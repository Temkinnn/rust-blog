use uuid::Uuid;

use crate::{
    errors::AppError,
    models::post::{CreatePostDto, CreatePostRepoDto, Post, PublicPost, UpdatePostDto},
    repositories::post::PostRepository,
    types::{AppResult},
};

pub struct PostService(PostRepository);

impl PostService {
    pub fn new(repo: PostRepository) -> Self {
        Self(repo)
    }

    fn generate_slug(&self, title: &str) -> String {
        title.replace(" ", "-") // Simple function to generate slug
    }

    pub async fn create_post(&self, author_id: Uuid, dto: CreatePostDto) -> AppResult<Post> {
        let dto = CreatePostRepoDto {
            author_id,
            content: dto.content,
            published: dto.published,
            slug: self.generate_slug(&dto.title),
            title: dto.title,
            tags: dto.tags,
        };
        Ok(self.0.create_post(dto).await?)
    }

    pub async fn get_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<PublicPost>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        Ok(self.0.get_posts(limit, offset).await?)
    }

    pub async fn get_posts_by_author_id(
        &self,
        author_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<PublicPost>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        Ok(self
            .0
            .get_posts_by_author_id(author_id, limit, offset)
            .await?)
    }

    pub async fn get_post_by_slug(&self, slug: String) -> AppResult<PublicPost> {
        self.0
            .get_post_by_slug(slug)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn update_post(
        &self,
        post_id: Uuid,
        author_id: Uuid,
        dto: UpdatePostDto,
    ) -> AppResult<Post> {
        self.0
            .update_post_by_id(post_id, author_id, dto)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete_post(&self, post_id: Uuid) -> AppResult<Post> {
        self.0
            .delete_post_by_id(post_id)
            .await?
            .ok_or(AppError::NotFound)
    }
}
