use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

use super::errors::UrlError;
use super::models::{create_url, delete_url_by_short_code, get_all_urls, get_url_by_code, Url};

pub async fn shorten_url(
    pool: web::Data<SqlitePool>,
    web::Json(payload): web::Json<Url>,
) -> impl Responder {
    match create_url(&pool, &payload.short_code, &payload.original_url).await {
        Ok(url) => HttpResponse::Created().json(url),
        Err(UrlError::ShortCodeExists) => {
            HttpResponse::Conflict().body("Short code already exists")
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn redirect_to_original(
    pool: web::Data<SqlitePool>,
    short_code: web::Path<String>,
) -> impl Responder {
    let short_code = short_code.into_inner();
    match get_url_by_code(&pool, &short_code).await {
        Ok(url) => HttpResponse::Found()
            .append_header(("Location", url.original_url))
            .finish(),
        Err(_) => HttpResponse::NotFound().into(),
    }
}

pub async fn list_urls(pool: web::Data<SqlitePool>) -> impl Responder {
    match get_all_urls(&pool).await {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_url(
    pool: web::Data<SqlitePool>,
    short_code: web::Path<String>,
) -> impl Responder {
    let short_code = short_code.into_inner();
    match delete_url_by_short_code(&pool, &short_code).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
