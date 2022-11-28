use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn insert(&self) -> Result<(), sqlx::Error>;
}

pub struct RepositoryImpl {}

// #[async_trait]
// impl Repository for Impl {
// }
