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
            .await
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
