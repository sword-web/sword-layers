use axum::body::HttpBody;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response as AxumResponse;
use axum_responses::JsonResponse;

pub struct ContentTypeCheck;

impl ContentTypeCheck {
    pub async fn layer(req: Request, next: Next) -> Result<AxumResponse, JsonResponse> {
        let content_type = req
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok());

        let Some(content_type) = content_type else {
            return Err(
                JsonResponse::UnsupportedMediaType().message("Content-Type header is missing.")
            );
        };

        if req.body().size_hint().lower() == 0 {
            return Ok(next.run(req).await);
        }

        if content_type != "application/json" && !content_type.contains("multipart/form-data") {
            return Err(JsonResponse::UnsupportedMediaType().message(
                "Only application/json and multipart/form-data content types are supported.",
            ));
        }

        Ok(next.run(req).await)
    }
}
