use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::{error, info};
use crate::common::response::res::ApiError;
use crate::middleware::UserAuthentication;
use crate::repo;

pub async fn authorized_middleware(mut req: Request, next: Next) -> Result<Response,ApiError> {

    info!("authorized_middleware.................");
    if let Some(current_user) = req.extensions_mut().get::<UserAuthentication>() {
       let username = &current_user.username;

        let user = repo::user::find_by_username(username).await;

        return match user {
            Ok(user) => {
                if let Some(user) = user {
                    if user.admin == true {
                        return Ok(next.run(req).await);
                    }
                    error!("User {} is not admin", user.username);
                }
                Err(ApiError::new(401, "Unauthorized".to_string()))
            }
            Err(_) => {
                error!("Could not find user by username {}", username);
                Err(ApiError::new(401, "Unauthorized".to_string()))
            }
        }
    }
    error!("current user not found");
    Err(ApiError::new(401, "Unauthorized".to_string()))
}