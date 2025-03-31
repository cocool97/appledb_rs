use sea_orm::sea_query::{ForeignKey, Index};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

use crate::{
    m5_entitlement::Entitlement,
    m7_executable_operating_system_version::ExecutableOperatingSystemVersion,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExecutableEntitlement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExecutableEntitlement::ExecutableOperatingSystemVersionId)
                            .not_null()
                            .integer(),
                    )
                    .col(
                        ColumnDef::new(ExecutableEntitlement::EntitlementId)
                            .not_null()
                            .integer(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ExecutableEntitlement::Table,
                                ExecutableEntitlement::ExecutableOperatingSystemVersionId,
                            )
                            .to(
                                ExecutableOperatingSystemVersion::Table,
                                ExecutableOperatingSystemVersion::Id,
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ExecutableEntitlement::Table,
                                ExecutableEntitlement::EntitlementId,
                            )
                            .to(Entitlement::Table, Entitlement::Id),
                    )
                    .primary_key(
                        Index::create()
                            .table(ExecutableEntitlement::Table)
                            .col(ExecutableEntitlement::ExecutableOperatingSystemVersionId)
                            .col(ExecutableEntitlement::EntitlementId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExecutableEntitlement::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ExecutableEntitlement {
    Table,
    ExecutableOperatingSystemVersionId,
    EntitlementId,
}
