use uuid::Uuid;

use crate::{
    models::comments::{Comment, CreateCommentRepoDto, UpdateCommentRepoDto},
    types::{Database, DatabaseResult},
};

pub struct CommentRepository(Database);

impl CommentRepository {
    pub fn new(pool: Database) -> Self {
        Self(pool)
    }

    pub async fn get_comments_by_post(
        &self,
        post_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> DatabaseResult<Vec<Comment>> {
        sqlx::query_as!(
            Comment,
            "Select id, content, author_id, post_id, updated_at, created_at from comments
            Where post_id = $1
            Limit $2 Offset $3",
            post_id,
            limit,
            offset
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn create_comment(&self, dto: CreateCommentRepoDto) -> DatabaseResult<Comment> {
        sqlx::query_as!(
            Comment,
            "Insert into comments (content, author_id, post_id)
            values ($1, $2, $3)
            Returning id, content, author_id, post_id, updated_at, created_at",
            dto.content,
            dto.author_id,
            dto.post_id
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn update_comment(
        &self,
        dto: UpdateCommentRepoDto,
    ) -> DatabaseResult<Option<Comment>> {
        sqlx::query_as!(
            Comment,
            "Update comments
                Set content = Coalesce($1, content)
            Where author_id = $2 AND id = $3
            Returning id, content, author_id, post_id, updated_at, created_at",
            dto.content,
            dto.author_id,
            dto.post_id
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn delete_comment(&self, comment_id: Uuid) -> DatabaseResult<Option<Comment>> {
        sqlx::query_as!(
            Comment,
            "Delete from comments
            Where id = $1
            Returning id, content, author_id, post_id, updated_at, created_at",
            comment_id
        )
        .fetch_optional(&self.0)
        .await
    }
}
