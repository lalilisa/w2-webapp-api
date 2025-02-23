use crate::middleware::UserAuthentication;
use crate::common;
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::StatusCode;
use tracing::{error, info};
use crate::common::response::res::ApiError;

pub async fn authentication_middleware(mut req: Request, next: Next) -> Result<Response<Body>, ApiError> {

    info!("authen_middleware.................");
    let headers = req.headers();
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    if let Some(auth) = auth_header {
        if auth.starts_with("Bearer ") {
            let token = auth.trim_start_matches("Bearer ");

            let res = match common::jwt::verify_jwt(token) {
                Ok(token_data) => {
                    let authenticated_user = UserAuthentication {
                        user_id: token_data.user_id,
                        username: token_data.username,
                    };

                    req.extensions_mut().insert(authenticated_user); // Store user data in request
                    Ok::<http::Response<Body>, ApiError>(next.run(req).await)
                }
                Err(_e) => {
                    error!("{}", _e);
                    return Err(ApiError::new(StatusCode::UNAUTHORIZED.as_u16(),"UNAUTHORIZED".to_string()));
                }
            };

            return res
        }

        return Err(ApiError::new(StatusCode::UNAUTHORIZED.as_u16(),"UNAUTHORIZED".to_string()))
    }
    error!("token is not valid");
    Err(ApiError::new(StatusCode::UNAUTHORIZED.as_u16(),"UNAUTHORIZED".to_string()))
}
