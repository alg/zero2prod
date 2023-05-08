use crate::authentication::UserId;
use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::admin::dashboard::get_username;
use crate::utils::{e500, see_other};
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use secrecy::ExposeSecret;
use secrecy::Secret;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let new_password = form.new_password.expose_secret();
    let new_password_check = form.new_password_check.expose_secret();
    let password_length = new_password.graphemes(true).count();
    let error_message: Option<&str>;

    if new_password != new_password_check {
        error_message =
            Some("You entered two different new passwords - the field values must match.");
    } else if !(12..=128).contains(&password_length) {
        error_message = Some("Password length must be between 12 and 128 characters.");
    } else {
        error_message = None;
    }

    if let Some(message) = error_message {
        FlashMessage::error(message).send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }

            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    FlashMessage::error("Your password has been changed.").send();

    Ok(see_other("/admin/password"))
}
