mod ingestors;

use std::{fs::File, net::TcpListener};

use diesel::{sql_query, RunQueryDsl as _};
use diesel_migrations::MigrationHarness as _;
use uuid::Uuid;

use config::Config;

pub struct TestApp {
    local_addr: String,
    client:     reqwest::Client,
}

impl TestApp {
    pub async fn spawn() -> Self {
        logging::init("NIBBLE_TEST_LOG").unwrap();

        let config_file = File::open("src/server/tests/config.yaml")
            .expect("failed to open config file");
        let mut config = serde_yaml::from_reader(config_file)
            .expect("failed to deserialize config");

        Self::configure_database(&mut config);

        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port.");
        let server =
            server::spawn(listener, config).expect("failed to spawn server");
        let local_addr = server.local_addr().to_string();

        tokio::task::spawn(async {
            let _ = server.await;
        });

        Self {
            local_addr,
            client: reqwest::Client::default(),
        }
    }

    pub fn ingestors(&self) -> ingestors::Scope {
        ingestors::Scope {
            parent:     self,
            url_prefix: format!("http://{}/ingestors", self.local_addr),
        }
    }

    fn configure_database(config: &mut Config) {
        let database = Uuid::new_v4().to_string();
        let mut conn = config
            .postgresql
            .maintenance
            .conn_blocking()
            .expect("failed to connect to mainteance database");

        sql_query(format!(
            r#"CREATE DATABASE "{}" OWNER "{}";"#,
            &database, &config.postgresql.main.user,
        ))
        .execute(&mut conn)
        .expect("failed to create test database");

        config.postgresql.main.database = database;

        let migrations = diesel_migrations::FileBasedMigrations::from_path(
            "src/server/migrations",
        )
        .expect("failed to create migration");
        let mut conn = config
            .postgresql
            .main
            .conn_blocking()
            .expect("failed to connect to test database");

        conn.run_pending_migrations(migrations)
            .expect("failed to run migrations");
    }
}
