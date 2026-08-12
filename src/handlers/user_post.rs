use actix_web::{HttpResponse, Responder, get, middleware::from_fn, web};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use uuid::Uuid;

use crate::{
    config::services::Services,
    middlewares::auth::auth_middleware,
    models::{post::Post, query::LimitOffsetQuery},
    types::AppResult,
};

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("User id" = Uuid, Path, description = "The Unique user Id"),
    ),
    responses(
        (status = 200, body = [Post]),
        (status = 401),
    )
)]
#[get("/users/{user_id}/posts/")]
async fn get_posts_by_author(
    services: web::Data<Services>,
    path: web::Path<Uuid>,
    query: web::Query<LimitOffsetQuery>,
) -> AppResult<impl Responder> {
    let posts = services
        .posts
        .get_posts_by_author_id(path.into_inner(), query.limit, query.offset)
        .await?;

    Ok(HttpResponse::Ok().json(posts))
}

pub fn users_posts_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .wrap(from_fn(auth_middleware))
            .service(get_posts_by_author),
    );
}
