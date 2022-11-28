use axum::{routing::post, Json, Router};

use error::Error;
use view::source::CreateRequestBody;

use crate::AppState;

pub fn router<S>(app_state: AppState) -> Router<S> {
    Router::new().route("/", post(create)).with_state(app_state)
}

#[tracing::instrument(name = "HANDLERS__SOURCES__CREATE", ret, err)]
async fn create(Json(params): Json<CreateRequestBody>) -> Result<(), Error> {
    Ok(())
}
