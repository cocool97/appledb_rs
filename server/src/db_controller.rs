use std::path::Path;

use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxSqliteConnector;
use sea_orm::sqlx::ConnectOptions;
use sea_orm::sqlx::sqlite::SqlitePoolOptions;
use sea_orm::{
    DatabaseConnection,
    sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};

const SQLDB_MIN_CONNECTIONS: u32 = 10;
const SQLDB_MAX_CONNECTIONS: u32 = 100;

pub struct DBController {
    connection: DatabaseConnection,
}

impl DBController {
    pub async fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let options = SqliteConnectOptions::default()
            .create_if_missing(true)
            .filename(database_path)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true)
            .disable_statement_logging();

        let pool = SqlitePoolOptions::new()
            .min_connections(SQLDB_MIN_CONNECTIONS)
            .max_connections(SQLDB_MAX_CONNECTIONS)
            .connect_with(options)
            .await?;

        let connection = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

        Migrator::up(&connection, None).await?;

        let controller = Self { connection };

        // Feed unknown Apple models display_name
        controller.crud_devices_set_unknown_display_names().await?;

        Ok(controller)
    }

    pub(crate) fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
