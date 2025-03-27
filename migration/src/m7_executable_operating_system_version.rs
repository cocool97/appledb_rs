use sea_orm::sea_query::{ForeignKey, Index};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

use crate::{m3_operating_system_version::OperatingSystemVersion, m4_executable::Executable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExecutableOperatingSystemVersion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExecutableOperatingSystemVersion::Id)
                            .not_null()
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExecutableOperatingSystemVersion::ExecutableId)
                            .not_null()
                            .integer(),
                    )
                    .col(
                        ColumnDef::new(ExecutableOperatingSystemVersion::OperatingSystemVersionId)
                            .not_null()
                            .integer(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ExecutableOperatingSystemVersion::Table,
                                ExecutableOperatingSystemVersion::ExecutableId,
                            )
                            .to(Executable::Table, Executable::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ExecutableOperatingSystemVersion::Table,
                                ExecutableOperatingSystemVersion::OperatingSystemVersionId,
                            )
                            .to(OperatingSystemVersion::Table, OperatingSystemVersion::Id),
                    )
                    .index(
                        Index::create()
                            .table(ExecutableOperatingSystemVersion::Table)
                            .col(ExecutableOperatingSystemVersion::ExecutableId)
                            .col(ExecutableOperatingSystemVersion::OperatingSystemVersionId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ExecutableOperatingSystemVersion::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExecutableOperatingSystemVersion {
    Table,
    Id,
    ExecutableId,
    OperatingSystemVersionId,
}
