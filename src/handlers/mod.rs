pub mod feed;
pub mod posts;
pub mod pages;
pub mod search;
pub mod contact;

use axum::{
    routing::{get, post},
    Router,
};

use crate::db::DbPool;

pub fn public_routes() -> Router<DbPool> {
    Router::new()
        // Feed routes
        .route("/", get(feed::homepage))
        .route("/htmx/feed/", get(feed::feed_partial))
        .route("/browse/", get(feed::browse))
        .route("/category/:slug/", get(feed::category_feed))
        .route("/tags/:slug/", get(feed::tag_feed))
        // Post routes
        .route("/posts/:id/", get(posts::detail))
        // Search
        .route("/search/", get(search::search_page))
        // Contact
        .route("/contact/", get(contact::contact_page).post(contact::contact_submit))
        // Static pages (catch-all at end)
        .route("/:slug/", get(pages::page_detail))
}
