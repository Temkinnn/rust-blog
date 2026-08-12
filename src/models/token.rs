use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
