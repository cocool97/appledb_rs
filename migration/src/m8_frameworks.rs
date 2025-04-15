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
                    .table(Framework::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Framework::Id)
                            .not_null()
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Framework::FullPath)
                            .not_null()
                            .string()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Framework::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Framework {
    Table,
    Id,
    FullPath,
}
