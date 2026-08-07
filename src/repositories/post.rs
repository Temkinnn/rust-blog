use crate::models::post::{CreatePostRepoDto, Post, UpdatePostDto};
use crate::types::{Database, DatabaseResult, Id};

pub struct PostRepository(Database);

impl PostRepository {
    pub fn new(pool: Database) -> Self {
        Self(pool)
    }

    pub async fn create_post(&self, dto: CreatePostRepoDto) -> DatabaseResult<Post> {
        sqlx::query_as!(
            Post,
            "
            Insert into posts (title, slug, content, published, tags, author_id)
            values ($1, $2, $3, $4, $5, $6)
            Returning id, title, slug, content, published, tags, author_id
            ",
            dto.title,
            dto.slug,
            dto.content,
            dto.published,
            &dto.tags,
            dto.author_id
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_posts(&self, limit: i64, offset: i64) -> DatabaseResult<Vec<Post>> {
        sqlx::query_as!(
            Post,
            "
            Select id, title, slug, content, published, tags, author_id from posts
            Limit $1 Offset $2
            ",
            limit,
            offset
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn get_posts_by_author_id(&self, author_id: Id) -> DatabaseResult<Vec<Post>> {
        sqlx::query_as!(
            Post,
            "
            Select id, title, slug, content, published, tags, author_id from posts
            Where author_id = $1
            ",
            author_id
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn get_post_by_slug(&self, slug: String) -> DatabaseResult<Option<Post>> {
        sqlx::query_as!(
            Post,
            "
            Select id, title, slug, content, published, tags, author_id from posts
            Where slug = $1
            ",
            slug
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn update_post_by_id(
        &self,
        post_id: Id,
        dto: UpdatePostDto,
    ) -> DatabaseResult<Option<Post>> {
        sqlx::query_as!(
            Post,
            "
            Update posts
            Set
                title = Coalesce($2, title),
                content = Coalesce($3, content),
                published = Coalesce($4, published),
                tags = Coalesce($5, tags)
            Where id = $1
            Returning id, title, slug, content, published, tags, author_id
            ",
            post_id,
            dto.title,
            dto.content,
            dto.published,
            &dto.tags.unwrap_or(vec![])
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn delete_post_by_id(&self, post_id: Id) -> DatabaseResult<Option<Post>> {
        sqlx::query_as!(
            Post,
            "
            Delete from posts
            Where id = $1
            Returning id, title, slug, content, published, tags, author_id",
            post_id
        )
        .fetch_optional(&self.0)
        .await
    }
}
