mod config;
mod errors;
mod static_files;
mod new_user;


use crate::errors::CustomError;
use axum::response::Html;
use axum::routing::post;
use axum::{extract::Extension, routing::get, Router};
use dioxus::dioxus_core::VirtualDom;
use std::net::SocketAddr;
use web_pages::{
    render,
    users::{IndexPage, IndexPageProps},
};

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/static/*path", get(static_files::static_path))
        .route("/sign_up", post(new_user::process_form))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on... {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = render(VirtualDom::new_with_props(
        IndexPage,
        IndexPageProps { users },
    ));

    Ok(Html(html))
}

