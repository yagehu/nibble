use std::{net::TcpListener, sync::Arc};

use axum::{routing::IntoMakeService, Router};
use color_eyre::eyre::Error as E;
use diesel_async::pooled_connection::deadpool::Pool;
use hyper::server::conn::AddrIncoming;

use config::Config;
use controller::IngestorControllerImpl;
use handler::AppState;
use repository::IngestorRepositoryImpl;

pub type Server = axum::Server<AddrIncoming, IntoMakeService<Router>>;

pub fn spawn(listener: TcpListener, config: Config) -> Result<Server, E> {
    let db_conn_pool =
        Pool::builder(config.postgresql.main.conn_pool_config()).build()?;

    let repository_ingestor =
        Arc::new(IngestorRepositoryImpl::new(db_conn_pool));

    let controller_ingestor =
        Arc::new(IngestorControllerImpl::new(repository_ingestor));

    Ok(axum::Server::from_tcp(listener)?.serve(
        axum::Router::new()
            .nest(
                "/",
                handler::router(AppState {
                    controller_ingestor,
                }),
            )
            .into_make_service(),
    ))
}
