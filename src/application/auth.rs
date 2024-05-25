use axum::{extract::Request, middleware::Next, response::Response};

use http::{HeaderMap, StatusCode};
pub async fn auth_middlewere(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match has_token(&headers) {
        true => {
            let response = next.run(request).await;
            Ok(response)
        }
        false => Err(StatusCode::UNAUTHORIZED),
    }
}

fn has_token(headers: &HeaderMap) -> bool {
    match headers.get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.contains("Bearer "),
            Err(_) => false,
        },
        None => false,
    }
}
