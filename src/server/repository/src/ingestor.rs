use async_trait::async_trait;
use color_eyre::eyre::{Context, Error as E};
use diesel::insert_into;
use diesel_async::{
    pooled_connection::deadpool::Pool,
    AsyncPgConnection,
    RunQueryDsl,
};

use model::{schema::ingestor::dsl::ingestor, Ingestor};

#[async_trait]
pub trait Repository: Send + Sync {
    async fn insert(
        &self,
        params: model::ingestor::Params<'async_trait>,
    ) -> Result<Ingestor, E>;
}

pub struct RepositoryImpl {
    db_conn_pool: Pool<AsyncPgConnection>,
}

impl RepositoryImpl {
    pub fn new(db_conn_pool: Pool<AsyncPgConnection>) -> Self {
        Self { db_conn_pool }
    }
}

#[async_trait]
impl Repository for RepositoryImpl {
    #[tracing::instrument(
        name = "REPOSITORY__INGESTOR__INSERT",
        skip(self),
        ret,
        err(Debug)
    )]
    async fn insert(
        &self,
        params: model::ingestor::Params<'async_trait>,
    ) -> Result<Ingestor, E> {
        let mut conn = self
            .db_conn_pool
            .get()
            .await
            .wrap_err("failed to acquire connection from pool")?;

        insert_into(ingestor)
            .values(&params)
            .get_result(&mut conn)
            .await
            .wrap_err("failed to insert record")
    }
}
