pub enum UrlError {
    DatabaseError(sqlx::Error),
    ShortCodeExists,
}

impl From<sqlx::Error> for UrlError {
    fn from(err: sqlx::Error) -> Self {
        UrlError::DatabaseError(err)
    }
}
