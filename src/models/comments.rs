use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub author_id: Option<Uuid>,
    pub post_id: Uuid,
    pub updated_at: PrimitiveDateTime,
    pub created_at: PrimitiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCommentDto {
    #[validate(length(min = 1, max = 120, message = "Content must be 1-120 charachters long"))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCommentRepoDto {
    pub content: String,
    pub author_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCommentDto {
    #[validate(length(min = 1, max = 120, message = "Content must be 1-120 charachters long"))]
    pub content: String,
    pub author_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCommentRepoDto {
    pub content: String,
    pub author_id: Uuid,
    pub post_id: Uuid,
}
