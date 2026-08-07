use crate::{
    models::post::{CreatePostDto, CreatePostRepoDto, Post, UpdatePostDto},
    repositories::post::PostRepository,
    types::{AppResult, Id},
};

pub struct PostService(PostRepository);

impl PostService {
    pub fn new(repo: PostRepository) -> Self {
        Self(repo)
    }

    fn generate_slug(&self, title: &str) -> String {
        title.replace(" ", "-") // Simple function to generate slug
    }

    pub async fn create_post(&self, author_id: Id, dto: CreatePostDto) -> AppResult<Post> {
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

    pub async fn get_posts(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<Post>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        Ok(self.0.get_posts(limit, offset).await?)
    }

    pub async fn get_posts_by_author_id(&self, author_id: Id) -> AppResult<Vec<Post>> {
        Ok(self.0.get_posts_by_author_id(author_id).await?)
    }

    pub async fn get_post_by_slug(&self, slug: String) -> AppResult<Option<Post>> {
        Ok(self.0.get_post_by_slug(slug).await?)
    }

    pub async fn update_post(&self, author_id: Id, dto: UpdatePostDto) -> AppResult<Option<Post>> {
        Ok(self.0.update_post_by_id(author_id, dto).await?)
    }

    pub async fn delete_post(&self, post_id: Id) -> AppResult<Option<Post>> {
        Ok(self.0.delete_post_by_id(post_id).await?)
    }
}
