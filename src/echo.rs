use axum::{
    body::Body,
    http::{header, HeaderValue, Request, StatusCode},
    response::Response,
};

pub async fn echo(req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    let (parts, body) = req.into_parts();

    Response::builder()
        .header(
            header::CONTENT_TYPE,
            parts
                .headers
                .get(header::CONTENT_TYPE)
                .unwrap_or(&HeaderValue::from_str("application/octet-stream").unwrap()),
        )
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
