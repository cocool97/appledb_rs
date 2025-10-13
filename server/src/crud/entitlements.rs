use appledb_common::db_models::Entitlement;

use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_entitlements_count(&self) -> Result<u64, DbErr> {
        entity::prelude::Entitlement::find()
            .count(self.get_connection())
            .await
    }

    pub async fn crud_get_entitlements_for_executable(
        &self,
        executable_operating_system_id: i64,
    ) -> Result<Vec<Entitlement>, DbErr> {
        let entitlements = entity::prelude::Entitlement::find()
            .join(
                JoinType::LeftJoin,
                entity::entitlement::Relation::ExecutableEntitlement.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::executable_entitlement::Relation::ExecutableOperatingSystemVersion.def(),
            )
            .filter(
                entity::executable_operating_system_version::Column::Id
                    .eq(executable_operating_system_id),
            )
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Entitlement::from)
            .collect();

        Ok(entitlements)
    }

    pub async fn crud_get_or_create_entitlement<S: AsRef<str>>(
        &self,
        key: S,
        value: S,
    ) -> Result<DBStatus, DbErr> {
        let key = key.as_ref();
        let value = value.as_ref();

        if let Some(entitlement) = entity::prelude::Entitlement::find()
            .filter(entity::entitlement::Column::Key.eq(key))
            .filter(entity::entitlement::Column::Value.eq(value))
            .one(self.get_connection())
            .await?
        {
            // Already exists in DB
            return Ok(DBStatus::AlreadyExists(entitlement.id));
        }

        let entitlement = entity::entitlement::ActiveModel {
            id: ActiveValue::NotSet,
            key: ActiveValue::Set(key.to_string()),
            value: ActiveValue::Set(value.to_string()),
        };

        let res = entitlement.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }
}
