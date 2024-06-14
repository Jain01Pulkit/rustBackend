use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Option<Uuid>, 
    pub name: String,
    pub location: String,
    pub title: String,
}