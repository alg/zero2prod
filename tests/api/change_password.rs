use crate::helpers::{assert_is_redirect_to, spawn_app, TestApp};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_change_password().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
                "current_password": Uuid::new_v4().to_string(),
                "new_password": &new_password,
                "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    assert_error(
        &app,
        &app.test_user.password,
        &new_password,
        &another_new_password,
        "You entered two different new passwords - the field values must match.",
    )
    .await;
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    assert_error(
        &app,
        &wrong_password,
        &new_password,
        &new_password,
        "The current password is incorrect.",
    )
    .await;
}

#[tokio::test]
async fn short_passwords_are_rejected() {
    // Arrange
    let app = spawn_app().await;
    let error_message = "Password length must be between 12 and 128 characters.";

    // Too short
    let new_password = "*".repeat(11);
    assert_error(
        &app,
        &app.test_user.password,
        &new_password,
        &new_password,
        error_message,
    )
    .await;

    // Too long
    let new_password = "*".repeat(129);
    assert_error(
        &app,
        &app.test_user.password,
        &new_password,
        &new_password,
        error_message,
    )
    .await;
}

async fn assert_error(
    app: &TestApp,
    current_password: &str,
    new_password: &str,
    new_password_check: &str,
    error_message: &str,
) {
    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &current_password,
            "new_password": &new_password,
            "new_password_check": &new_password_check,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(error_message));
}
