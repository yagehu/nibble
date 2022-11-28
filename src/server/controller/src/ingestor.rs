use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::eyre::{Context, Error as E};
use time::OffsetDateTime;
use uuid::Uuid;

use entity::Ingestor;
use repository::IngestorRepository;

#[async_trait]
pub trait Controller: Send + Sync {
    async fn create(
        &self,
        params: CreateParams<'async_trait>,
    ) -> Result<Ingestor, E>;
}

#[derive(Clone)]
pub struct ControllerImpl {
    repository_ingestor: Arc<dyn IngestorRepository>,
}

impl ControllerImpl {
    pub fn new(repository_ingestor: Arc<dyn IngestorRepository>) -> Self {
        Self {
            repository_ingestor,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CreateParams<'a> {
    pub name:       &'a str,
    pub image_name: &'a str,
    pub image_tag:  &'a str,
}

#[async_trait]
impl Controller for ControllerImpl {
    #[tracing::instrument(
        name = "CONTROLLER__INGESTOR__CREATE",
        skip(self),
        ret,
        err
    )]
    async fn create(
        &self,
        params: CreateParams<'async_trait>,
    ) -> Result<Ingestor, E> {
        let id = Uuid::new_v4();
        let ingestor = self
            .repository_ingestor
            .insert(model::ingestor::Params {
                id,
                created_at: OffsetDateTime::now_utc(),
                name: params.name,
                image_name: params.image_name,
                image_tag: params.image_tag,
            })
            .await
            .wrap_err("failed to insert ingestor into database")?;

        Ok(ingestor.into())
    }
}
