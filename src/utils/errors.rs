use axum::response::IntoResponse;
use serde_json::json;

// Define the error response struct
#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    message: String,
}

// Implement IntoResponse for ApiError
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": self.message
        }));
        
        (self.status, body).into_response()
    }
}