use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::Form;

use crate::db::DbPool;
use crate::error::Result;
use crate::models::{ContactSubmission, CreateContactSubmission, Page, Profile};

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub nav_pages: Vec<Page>,
    pub current_path: String,
    pub success_message: Option<String>,
    pub error: Option<String>,
    pub user: Option<Profile>,
}

pub async fn contact_page(
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse> {
    let nav_pages = Page::list_nav_pages(&pool).await?;

    Ok(ContactTemplate {
        nav_pages,
        current_path: "/contact/".to_string(),
        success_message: None,
        error: None,
        user: None,
    })
}

pub async fn contact_submit(
    State(pool): State<DbPool>,
    Form(input): Form<CreateContactSubmission>,
) -> Result<impl IntoResponse> {
    let nav_pages = Page::list_nav_pages(&pool).await?;

    // Validate input
    if input.name.trim().is_empty() || input.email.trim().is_empty() || input.message.trim().is_empty() {
        return Ok(ContactTemplate {
            nav_pages,
            current_path: "/contact/".to_string(),
            success_message: None,
            error: Some("All fields are required.".to_string()),
            user: None,
        });
    }

    // Create submission
    match ContactSubmission::create(&pool, None, input, None, None).await {
        Ok(_) => Ok(ContactTemplate {
            nav_pages,
            current_path: "/contact/".to_string(),
            success_message: Some("Thank you for your message! I'll get back to you soon.".to_string()),
            error: None,
            user: None,
        }),
        Err(e) => {
            tracing::error!("Failed to save contact submission: {:?}", e);
            Ok(ContactTemplate {
                nav_pages,
                current_path: "/contact/".to_string(),
                success_message: None,
                error: Some("Sorry, there was an error. Please try again.".to_string()),
                user: None,
            })
        }
    }
}
