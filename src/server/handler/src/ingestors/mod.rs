use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json,
    Router,
};

use controller::ingestor::{Controller, CreateParams};
use error::Error;
use view::{ingestor::CreateRequestBody, Ingestor};

use crate::AppState;

pub fn router<S>(app_state: AppState) -> Router<S> {
    Router::new().route("/", post(create)).with_state(app_state)
}

#[tracing::instrument(
    name = "HANDLER__INGESTORS__CREATE",
    skip(controller),
    ret,
    err
)]
async fn create(
    State(controller): State<Arc<dyn Controller>>,
    Json(params): Json<CreateRequestBody>,
) -> Result<impl IntoResponse, Error> {
    controller
        .create(CreateParams {
            name:       &params.name,
            image_name: &params.image_name,
            image_tag:  &params.image_tag,
        })
        .await
        .map(view::Ingestor::from)
        .map(|ingestor| (StatusCode::CREATED, Json(ingestor)))
        .map_err(|_| Error::Unknown)
}
