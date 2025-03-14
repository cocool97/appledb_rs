use sea_orm::sea_query::{ForeignKey, Index};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

use crate::m4_executable::Executable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entitlement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Entitlement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Entitlement::Key).not_null().string())
                    .col(ColumnDef::new(Entitlement::Value).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExecutableEntitlement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExecutableEntitlement::ExecutableId)
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
                                ExecutableEntitlement::ExecutableId,
                            )
                            .to(Executable::Table, Executable::Id),
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
                            .col(ExecutableEntitlement::ExecutableId)
                            .col(ExecutableEntitlement::EntitlementId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entitlement::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Entitlement {
    Table,
    Id,
    Key,
    Value,
}

#[derive(DeriveIden)]
pub enum ExecutableEntitlement {
    Table,
    ExecutableId,
    EntitlementId,
}
