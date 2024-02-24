use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, SqlitePool};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Url {
    pub id: Option<i64>,
    pub short_code: String,
    pub original_url: String,
}

pub async fn create_url(
    pool: &SqlitePool,
    short_code: &str,
    original_url: &str,
) -> Result<Url, Error> {
    let mut conn = pool.acquire().await?;
    let url = sqlx::query_as!(
        Url,
        "INSERT INTO urls (short_code, original_url) VALUES (?, ?) RETURNING *",
        short_code,
        original_url
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(url)
}

pub async fn get_url_by_code(pool: &SqlitePool, short_code: &str) -> Result<Url, Error> {
    let mut conn = pool.acquire().await?;
    let url = sqlx::query_as!(Url, "SELECT * FROM urls WHERE short_code = ?", short_code)
        .fetch_one(&mut *conn)
        .await?;

    Ok(url)
}
