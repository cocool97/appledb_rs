//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "executable")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub full_path: String,
    pub name: String,
    pub operating_system_version_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::executable_entitlement::Entity")]
    ExecutableEntitlement,
    #[sea_orm(
        belongs_to = "super::operating_system_version::Entity",
        from = "Column::OperatingSystemVersionId",
        to = "super::operating_system_version::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    OperatingSystemVersion,
}

impl Related<super::executable_entitlement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExecutableEntitlement.def()
    }
}

impl Related<super::operating_system_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OperatingSystemVersion.def()
    }
}

impl Related<super::entitlement::Entity> for Entity {
    fn to() -> RelationDef {
        super::executable_entitlement::Relation::Entitlement.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::executable_entitlement::Relation::Executable
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
