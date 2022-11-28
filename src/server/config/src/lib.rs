use diesel::{result::DatabaseErrorKind, Connection, PgConnection};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection,
};
use rustls::RootCertStore;
use serde::Deserialize;
use tokio_postgres_rustls::MakeRustlsConnect;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub postgresql: PostgresqlConnections,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PostgresqlConnections {
    pub main:        Postgresql,
    pub maintenance: Postgresql,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Postgresql {
    pub host:     String,
    pub port:     u16,
    pub user:     String,
    pub password: String,
    pub database: String,
}

impl Postgresql {
    pub async fn conn(&self) -> diesel::ConnectionResult<AsyncPgConnection> {
        Self::conn_with_url(&self.conn_string()).await
    }

    pub fn conn_blocking(&self) -> diesel::ConnectionResult<PgConnection> {
        PgConnection::establish(&self.conn_string())
    }

    pub fn conn_pool_config(
        &self,
    ) -> AsyncDieselConnectionManager<AsyncPgConnection> {
        AsyncDieselConnectionManager::new_with_setup(
            self.conn_string(),
            |url| Box::pin(Self::conn_with_url(url)),
        )
    }

    fn conn_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?connect_timeout=2",
            self.user, self.password, self.host, self.port, self.database,
        )
    }

    async fn conn_with_url(
        url: &str,
    ) -> diesel::ConnectionResult<AsyncPgConnection> {
        let (client, conn) = tokio_postgres::connect(
            url,
            MakeRustlsConnect::new(
                rustls::ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(RootCertStore::empty())
                    .with_no_client_auth(),
            ),
        )
        .await
        .map_err(|err| {
            diesel::ConnectionError::CouldntSetupConfiguration(
                diesel::result::Error::DatabaseError(
                    DatabaseErrorKind::Unknown,
                    Box::new(err.to_string()) as _,
                ),
            )
        })?;

        tokio::spawn(async move {
            let _ = conn.await;
        });

        AsyncPgConnection::try_from(client).await
    }
}
