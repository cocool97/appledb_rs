use sea_orm::sea_query::{ForeignKey, Index};
use sea_orm_migration::{
    DbErr, MigrationTrait, SchemaManager, async_trait,
    prelude::{ColumnDef, Table},
    sea_orm::{DeriveIden, DeriveMigrationName},
};

use crate::{m1_operating_system::OperatingSystem, m2_device::Device};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OperatingSystemVersion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OperatingSystemVersion::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OperatingSystemVersion::Version)
                            .not_null()
                            .string(),
                    )
                    .col(
                        ColumnDef::new(OperatingSystemVersion::OperatingSystemId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OperatingSystemVersion::DeviceId)
                            .big_integer()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .table(OperatingSystemVersion::Table)
                            .col(OperatingSystemVersion::Version)
                            .col(OperatingSystemVersion::OperatingSystemId)
                            .col(OperatingSystemVersion::DeviceId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                OperatingSystemVersion::Table,
                                OperatingSystemVersion::OperatingSystemId,
                            )
                            .to(OperatingSystem::Table, OperatingSystem::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                OperatingSystemVersion::Table,
                                OperatingSystemVersion::DeviceId,
                            )
                            .to(Device::Table, Device::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OperatingSystemVersion::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum OperatingSystemVersion {
    Table,
    Id,
    Version,
    OperatingSystemId,
    DeviceId,
}
