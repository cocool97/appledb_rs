use sea_orm::sea_query::Index;
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
                    .table(Executable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Executable::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Executable::FullPath).not_null().string())
                    .col(ColumnDef::new(Executable::Name).not_null().string())
                    .index(
                        Index::create()
                            .table(Executable::Table)
                            .col(Executable::Name)
                            .col(Executable::FullPath)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Executable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Executable {
    Table,
    Id,
    FullPath,
    Name,
}
