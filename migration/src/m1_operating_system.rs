use entity::operating_system::Column;
use sea_orm::{ActiveValue, EntityTrait, sea_query::OnConflict};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OperatingSystem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OperatingSystem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OperatingSystem::Name)
                            .not_null()
                            .unique_key()
                            .string(),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        // Pre-Create known Apple operating systems names
        entity::prelude::OperatingSystem::insert_many([
            entity::operating_system::ActiveModel {
                id: ActiveValue::not_set(),
                name: ActiveValue::Set("ios".to_string()),
            },
            entity::operating_system::ActiveModel {
                id: ActiveValue::not_set(),
                name: ActiveValue::Set("macos".to_string()),
            },
            entity::operating_system::ActiveModel {
                id: ActiveValue::not_set(),
                name: ActiveValue::Set("watchos".to_string()),
            },
            entity::operating_system::ActiveModel {
                id: ActiveValue::not_set(),
                name: ActiveValue::Set("tvos".to_string()),
            },
        ])
        .on_conflict(OnConflict::column(Column::Name).do_nothing().to_owned())
        .exec(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OperatingSystem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OperatingSystem {
    Table,
    Id,
    Name,
}
