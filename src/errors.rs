use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AppErrorText<'a> {
    ServerError(Option<&'a str>),
    NotFound(Option<&'a str>),
}

impl IntoResponse for AppErrorText<'_> {
    fn into_response(self) -> axum::response::Response {
        let (r_status, err_msg) = match self {
            Self::NotFound(info_msg) => {
                let tmp_msg = match info_msg {
                    Some(msg) => msg,
                    _ => "",
                };

                (
                    StatusCode::NOT_FOUND,
                    format!("Page {} not found!", tmp_msg),
                )
            }

            Self::ServerError(info_msg) => {
                let tmp_msg = match info_msg {
                    Some(msg) => format!(" Error: {}", msg),
                    _ => "".to_owned(),
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal failure.{}", tmp_msg),
                )
            }
        };

        (r_status, Json(json!({ "error": err_msg }))).into_response()
    }
}
