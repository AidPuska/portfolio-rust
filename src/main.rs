mod handlers;

use dotenv::dotenv;
use hyper::Method;
use tower_http::cors::CorsLayer;
use std::env;
use mongodb::Client;
use axum::{routing::get, Router};
use handlers::{add_post, delete_post, get_posts, returns_views, update_and_return};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();
    let db_url = env::var("DB_URL").unwrap_or_default();
    let client = Client::with_uri_str(db_url).await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(["localhost:*".parse().unwrap(), "https://apuska.netlify.app/".parse().unwrap()]);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/views", get(returns_views).post(update_and_return))
        .route("/api/posts", get(get_posts).post(add_post).delete(delete_post))
        .with_state(client)
        .layer(cors);

    Ok(app.into())
}

async fn root() -> &'static str {
    "Portfolio api"
}