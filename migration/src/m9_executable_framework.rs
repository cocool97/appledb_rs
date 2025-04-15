use sea_orm::sea_query::{ForeignKey, Index};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

use crate::{
    m6_executable_operating_system_version::ExecutableOperatingSystemVersion,
    m8_frameworks::Framework,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExecutableFramework::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExecutableFramework::ExecutableOperatingSystemVersionId)
                            .not_null()
                            .integer(),
                    )
                    .col(
                        ColumnDef::new(ExecutableFramework::FrameworkId)
                            .not_null()
                            .integer(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ExecutableFramework::Table,
                                ExecutableFramework::ExecutableOperatingSystemVersionId,
                            )
                            .to(
                                ExecutableOperatingSystemVersion::Table,
                                ExecutableOperatingSystemVersion::Id,
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExecutableFramework::Table, ExecutableFramework::FrameworkId)
                            .to(Framework::Table, Framework::Id),
                    )
                    .primary_key(
                        Index::create()
                            .table(ExecutableFramework::Table)
                            .col(ExecutableFramework::ExecutableOperatingSystemVersionId)
                            .col(ExecutableFramework::FrameworkId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExecutableFramework::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExecutableFramework {
    Table,
    ExecutableOperatingSystemVersionId,
    FrameworkId,
}
