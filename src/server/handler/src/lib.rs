mod ingestors;
mod sources;

use std::sync::Arc;

use axum::{extract::FromRef, Router};

use controller::IngestorController;

#[derive(Clone)]
pub struct AppState {
    pub controller_ingestor: Arc<dyn IngestorController>,
}

impl FromRef<AppState> for Arc<dyn IngestorController + 'static> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.controller_ingestor.clone()
    }
}

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .nest("/ingestors", ingestors::router(app_state.clone()))
        .nest("/sources", sources::router(app_state))
}
