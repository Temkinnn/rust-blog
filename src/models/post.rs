use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub author_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicPost {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePostDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Password must be at least 1 character long"
    ))]
    pub title: String,
    #[validate(length(min = 1, message = "Password must be at least 1 character long"))]
    pub content: String,
    pub published: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePostRepoDto {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub author_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePostDto {
    #[validate(length(min = 1, message = "Password must be at least 1 character long"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "Password must be at least 1 character long"))]
    pub content: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<String>>,
}
