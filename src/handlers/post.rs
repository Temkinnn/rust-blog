use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, middleware::from_fn, patch, post, web,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use uuid::Uuid;

use crate::{
    config::services::Services,
    errors::AppError,
    middlewares::auth::auth_middleware,
    models::{
        post::{CreatePostDto, Post, UpdatePostDto},
        query::LimitOffsetQuery,
    },
    types::{AppResult, Id},
};

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, body = Post),
        (status = 401),
    )
)]
#[post("/{slug}")]
async fn create_post(
    req: HttpRequest,
    services: web::Data<Services>,
    body: web::Json<CreatePostDto>,
) -> AppResult<impl Responder> {
    use actix_web::HttpMessage;

    let author_id = *req
        .extensions()
        .get::<Uuid>()
        .ok_or(AppError::Unauthorized)?;

    let post = services
        .posts
        .create_post(author_id, body.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(post))
}

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [Post]),
        (status = 401),
    )
)]
#[get("/")]
async fn get_posts(
    services: web::Data<Services>,
    query: web::Query<LimitOffsetQuery>,
) -> AppResult<impl Responder> {
    let posts = services.posts.get_posts(query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [Post]),
        (status = 401),
    )
)]
#[get("/my")]
async fn get_my_posts(
    req: HttpRequest,
    services: web::Data<Services>,
    query: web::Query<LimitOffsetQuery>,
) -> AppResult<impl Responder> {
    use actix_web::HttpMessage;

    let author_id = *req
        .extensions()
        .get::<Uuid>()
        .ok_or(AppError::Unauthorized)?;

    let posts = services
        .posts
        .get_posts_by_author_id(author_id, query.limit, query.offset)
        .await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("slug" = String, Path, description = "The slug to the post"),
    ),
    responses(
        (status = 200, body = Post),
        (status = 401),
        (status = 404),
    )
)]
#[get("/{slug}")]
async fn get_post_by_slug(
    services: web::Data<Services>,
    path: web::Path<String>,
) -> AppResult<impl Responder> {
    let post = services.posts.get_post_by_slug(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "The Unique post Id"),
    ),
    responses(
        (status = 200, body = Post),
        (status = 401),
        (status = 404),
    )
)]
#[patch("/{id}")]
async fn update_post(
    services: web::Data<Services>,
    path: web::Path<Id>,
    dto: web::Json<UpdatePostDto>,
) -> AppResult<impl Responder> {
    let post = services
        .posts
        .update_post(path.into_inner(), dto.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(post))
}

#[utoipa::path(
    tag = "Post",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "The Unique post Id"),
    ),
    responses(
        (status = 200, body = Post),
        (status = 401),
        (status = 404),
    )
)]
#[delete("/{id}")]
async fn delete_post(
    services: web::Data<Services>,
    path: web::Path<Id>,
    dto: web::Json<UpdatePostDto>,
) -> AppResult<impl Responder> {
    let post = services
        .posts
        .update_post(path.into_inner(), dto.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(post))
}

pub fn posts_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/posts")
            .wrap(from_fn(auth_middleware))
            .service(create_post)
            .service(get_posts)
            .service(get_my_posts)
            .service(get_post_by_slug)
            .service(update_post)
            .service(delete_post),
    );
}
