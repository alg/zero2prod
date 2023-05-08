use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Send newsletter</title>
</head>
<body>
    {error_html}

    <form action="/admin/newsletters" method="post">
        <label>
            Title
            <input type="text" placeholder="Title" name="title">
        </label>
        <br>

        <label>HTML Content</label>
        <textarea name="content[html]"></textarea>
        <br>

        <label>Text Content</label>
        <textarea name="content[text]"></textarea>
        <br>

        <button type="submit">Send</button>
    </form>

    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
