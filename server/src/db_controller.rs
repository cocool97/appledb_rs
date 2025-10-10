use std::time::Duration;

use anyhow::{Result, bail};
use log::LevelFilter;
use migration::{Migrator, MigratorTrait};
use sea_orm::sqlx::ConnectOptions;
use sea_orm::sqlx::sqlite::SqlitePoolOptions;
use sea_orm::{Database, DbErr, SqlxSqliteConnector};
use sea_orm::{
    DatabaseConnection,
    sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};
use url::Url;

const POOL_MIN_CONNECTIONS: u32 = 10;
const POOL_MAX_CONNECTIONS: u32 = 100;

pub struct DBController {
    connection: DatabaseConnection,
}

impl DBController {
    pub async fn new<S: AsRef<str>>(database_url: S) -> Result<Self> {
        let url = Url::parse(database_url.as_ref())?;
        let connection = match url.scheme() {
            "sqlite" => {
                log::info!("sqlite database backend selected");
                Self::get_sqlite_connection(url).await?
            }
            "postgres" => {
                log::info!("postgres database backend selected");
                Self::get_postgres_connection(url).await?
            }
            scheme => bail!("unknown scheme {scheme}"),
        };

        // Run needed migrations
        Migrator::up(&connection, None).await?;

        let controller = Self { connection };

        // Try to feed unknown Apple models display_name
        controller.crud_set_devices_unknown_display_names().await?;

        Ok(controller)
    }

    async fn get_sqlite_connection(url: Url) -> Result<DatabaseConnection> {
        let Some(database_path) = url.host_str() else {
            bail!("sqlite url without a path...");
        };

        let options = SqliteConnectOptions::default()
            .create_if_missing(true)
            .filename(database_path)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true)
            .in_memory(false)
            .busy_timeout(Duration::from_secs(5))
            .disable_statement_logging();

        let pool = SqlitePoolOptions::new()
            .min_connections(POOL_MIN_CONNECTIONS)
            .max_connections(POOL_MAX_CONNECTIONS)
            .connect_with(options)
            .await?;

        Ok(SqlxSqliteConnector::from_sqlx_sqlite_pool(pool))
    }

    async fn get_postgres_connection(url: Url) -> Result<DatabaseConnection, DbErr> {
        let mut options = sea_orm::ConnectOptions::new(url);

        options
            .min_connections(POOL_MIN_CONNECTIONS)
            .max_connections(POOL_MAX_CONNECTIONS)
            .connect_timeout(Duration::from_secs(5))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(10))
            .sqlx_logging(false)
            .sqlx_logging_level(LevelFilter::Info);

        Database::connect(options).await
    }

    pub(crate) fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
