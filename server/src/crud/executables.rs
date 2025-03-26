use std::{path::PathBuf, str::FromStr};

use appledb_common::db_models::Executable;

use anyhow::{Result, anyhow};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

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
        full_path: S,
    ) -> Result<DBStatus> {
        let executable_name = full_path.to_string();
        let executable_name = PathBuf::from_str(executable_name.as_str())?;

        let executable_name = executable_name.file_name().ok_or(anyhow!(
            "cannot get file name from path {}",
            full_path.to_string()
        ))?;

        if let Some(executable) = entity::prelude::Executable::find()
            .filter(entity::executable::Column::Name.eq(executable_name.to_string_lossy()))
            .filter(entity::executable::Column::FullPath.eq(full_path.to_string()))
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
            full_path: ActiveValue::Set(full_path.to_string()),
            name: ActiveValue::Set(executable_name.to_string_lossy().to_string()),
        };

        let res = executable.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }
}
