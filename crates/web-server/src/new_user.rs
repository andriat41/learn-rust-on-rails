use serde::Deserialize;
use validator::Validate;

use crate::errors::CustomError;
use axum::{
    extract::Extension, http::StatusCode, response::{IntoResponse, Redirect, Response}, Form
};

#[derive(Deserialize, Validate)]
pub struct SignUp {
    #[validate(email)]
    email: String,
}

// 👇 handle form submission
pub async fn process_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Response, CustomError> {
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request").into_response());
    }
    
    let client = pool.get().await?;

    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response())
}
