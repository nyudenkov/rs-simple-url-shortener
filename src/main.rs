use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::SqlitePool;

mod auth;
mod errors;
mod models;
mod routes;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        let auth_md = HttpAuthentication::bearer(auth::validator);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/public")
                    .route("/{short_code}", web::get().to(routes::redirect_to_original)),
            )
            .service(
                web::scope("/secure")
                    .wrap(auth_md)
                    .route("/shorten", web::post().to(routes::shorten_url))
                    .route("/urls", web::get().to(routes::list_urls)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
