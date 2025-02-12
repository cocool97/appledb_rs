use appledb_common::db_models::Executable;

use anyhow::{Result, anyhow};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait,
};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_executables(&self) -> Result<Vec<Executable>> {
        Ok(entity::prelude::Executable::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Executable::from)
            .collect::<Vec<Executable>>())
    }

    pub async fn crud_get_executable_by_id(&self, id: i32) -> Result<Executable> {
        let executable = entity::prelude::Executable::find_by_id(id)
            .one(self.get_connection())
            .await?
            .ok_or(anyhow!("unknown executable id {id}"))?;

        Ok(executable.into())
    }

    pub async fn crud_get_executables_by_name(&self, name: String) -> Result<Vec<Executable>> {
        let executables = entity::prelude::Executable::find()
            .filter(entity::executable::Column::Name.eq(name))
            .all(self.get_connection())
            .await?;

        Ok(executables.into_iter().map(|v| v.into()).collect())
    }

    pub async fn crud_get_or_create_executable<S: ToString>(
        &self,
        operating_system_version_id: i32,
        name: S,
    ) -> Result<DBStatus> {
        if let Some(executable) = entity::prelude::Executable::find()
            .filter(entity::executable::Column::Name.eq(name.to_string()))
            .filter(
                entity::executable::Column::OperatingSystemVersionId
                    .eq(operating_system_version_id),
            )
            .one(self.get_connection())
            .await?
        {
            // Already exists in DB
            return Ok(DBStatus::AlreadyExists(executable.id));
        }

        let executable = entity::executable::ActiveModel {
            id: ActiveValue::NotSet,
            operating_system_version_id: ActiveValue::set(operating_system_version_id),
            name: ActiveValue::Set(name.to_string()),
        };

        let res = executable.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }

    pub async fn crud_get_executables_with_entitlement_for_os_version<S: ToString>(
        &self,
        operating_system_version_id: i32,
        entitlement_key: S,
    ) -> Result<Vec<Executable>, DbErr> {
        let entitlements = entity::prelude::Entitlement::find()
            .filter(entity::entitlement::Column::Key.eq(entitlement_key.to_string()))
            .all(self.get_connection())
            .await?;

        let entitlement_ids: Vec<i32> = entitlements.into_iter().map(|e| e.id).collect();

        if entitlement_ids.is_empty() {
            return Ok(vec![]);
        }

        // Fetch executables with matching OS version and entitlement
        let executables = entity::prelude::Executable::find()
            .join(
                JoinType::LeftJoin,
                entity::executable::Relation::ExecutableEntitlement.def(),
            )
            .join(
                JoinType::LeftJoin,
                entity::executable_entitlement::Relation::Entitlement.def(),
            )
            .filter(
                entity::executable::Column::OperatingSystemVersionId
                    .eq(operating_system_version_id),
            )
            .filter(entity::entitlement::Column::Key.contains(entitlement_key.to_string()))
            .distinct()
            .into_model::<entity::executable::Model>()
            .all(self.get_connection())
            .await?;

        Ok(executables.into_iter().map(|v| v.into()).collect())
    }
}
