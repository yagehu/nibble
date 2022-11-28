use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    thiserror::Error, Serialize, Deserialize, PartialEq, Eq, Clone, Debug,
)]
pub enum Error {
    #[error("Unknown error.")]
    Unknown,
}

impl From<Error> for ResponseBody {
    fn from(x: Error) -> Self {
        match x {
            | _ => Self {
                error: x,
                msg:   "Unknown error.".to_owned(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ResponseBody {
    pub error: Error,
    pub msg:   String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            | _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseBody::from(self)),
            )
                .into_response(),
        }
    }
}
