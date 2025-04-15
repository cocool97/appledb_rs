use std::path::Path;

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_or_create_framework<P: AsRef<Path>>(
        &self,
        framework_full_path: P,
    ) -> Result<DBStatus, DbErr> {
        let framework_full_path = framework_full_path.as_ref();
        let new_framework = entity::framework::ActiveModel {
            id: ActiveValue::NotSet,
            full_path: ActiveValue::Set(framework_full_path.display().to_string()),
        };

        match new_framework.insert(self.get_connection()).await {
            Ok(inserted) => Ok(DBStatus::Created(inserted.id)),
            Err(DbErr::Exec(_)) => {
                let existing = entity::prelude::Framework::find()
                    .filter(
                        entity::framework::Column::FullPath
                            .eq(framework_full_path.to_string_lossy()),
                    )
                    .one(self.get_connection())
                    .await?
                    .ok_or_else(|| {
                        DbErr::Custom("Failed to retrieve after unique constraint violation".into())
                    })?;

                Ok(DBStatus::AlreadyExists(existing.id))
            }
            Err(e) => Err(e),
        }
    }
}
