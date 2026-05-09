use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{AppState, auth::utils::validate_token};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Get the Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

    // Check it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization format. Use: Bearer <token>".to_string(),
        ));
    }

    // Extract the token
    let token = &auth_header["Bearer ".len()..];

    // Validate the token
    let claims = validate_token(token, &state.jwt_secret)
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    // Insert claims into request extensions so handlers can access them
    request.extensions_mut().insert(claims);

    // Pass the request to the next handler
    Ok(next.run(request).await)
}