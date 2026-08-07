use crate::{
    repositories::{post::PostRepository, token::TokenRepository, user::UserRepository},
    types::{Database, Redis},
};

pub struct Repositories {
    pub token: TokenRepository,
    pub user: UserRepository,
    pub post: PostRepository,
}

impl Repositories {
    pub fn init(db: Database, redis: Redis) -> Self {
        Self {
            token: TokenRepository::new(redis),
            user: UserRepository::new(db.clone()),
            post: PostRepository::new(db.clone()),
        }
    }
}
