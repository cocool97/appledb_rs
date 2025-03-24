use appledb_common::db_models::Entitlement;

use anyhow::{Result, anyhow};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter,
};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_entitlements(&self) -> Result<Vec<Entitlement>> {
        Ok(entity::prelude::Entitlement::find()
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Entitlement::from)
            .collect::<Vec<Entitlement>>())
    }

    pub async fn crud_get_entitlements_count(&self) -> Result<u64> {
        Ok(entity::prelude::Entitlement::find()
            .count(self.get_connection())
            .await?)
    }

    pub async fn crud_get_entitlement_by_id(&self, id: i32) -> Result<Entitlement> {
        let entitlement = entity::prelude::Entitlement::find_by_id(id)
            .one(self.get_connection())
            .await?
            .ok_or(anyhow!("unknown entitlement id {id}"))?;

        Ok(entitlement.into())
    }

    pub async fn crud_get_entitlements_by_name(&self, name: String) -> Result<Vec<Entitlement>> {
        let entitlements = entity::prelude::Entitlement::find()
            .filter(entity::entitlement::Column::Key.contains(name))
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Entitlement::from)
            .collect();

        Ok(entitlements)
    }

    pub async fn crud_get_entitlements_for_executable(
        &self,
        executable_id: i32,
    ) -> Result<Vec<Entitlement>> {
        let executable = entity::prelude::Executable::find_by_id(executable_id)
            .one(self.get_connection())
            .await?
            .ok_or(anyhow!("unknown executable id {executable_id}"))?;

        let entitlements = executable
            .find_related(entity::prelude::Entitlement)
            .all(self.get_connection())
            .await?
            .into_iter()
            .map(Entitlement::from)
            .collect();

        Ok(entitlements)
    }

    pub async fn crud_get_or_create_entitlement<S: ToString>(
        &self,
        key: S,
        value: S,
    ) -> Result<DBStatus> {
        if let Some(entitlement) = entity::prelude::Entitlement::find()
            .filter(entity::entitlement::Column::Key.eq(key.to_string()))
            .filter(entity::entitlement::Column::Value.eq(value.to_string()))
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
