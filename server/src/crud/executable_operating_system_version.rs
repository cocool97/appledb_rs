use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_or_create_executable_operating_system_version(
        &self,
        executable_id: i32,
        operating_system_version_id: i32,
    ) -> Result<DBStatus, DbErr> {
        if let Some(executable_os) = entity::prelude::ExecutableOperatingSystemVersion::find()
            .filter(
                entity::executable_operating_system_version::Column::ExecutableId.eq(executable_id),
            )
            .filter(
                entity::executable_operating_system_version::Column::OperatingSystemVersionId
                    .eq(operating_system_version_id),
            )
            .one(self.get_connection())
            .await?
        {
            // Already exists
            return Ok(DBStatus::AlreadyExists(executable_os.id));
        }

        let executable_os = entity::executable_operating_system_version::ActiveModel {
            id: ActiveValue::NotSet,
            executable_id: ActiveValue::Set(executable_id),
            operating_system_version_id: ActiveValue::Set(operating_system_version_id),
        };

        let inserted = executable_os.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(inserted.id))
    }
}
