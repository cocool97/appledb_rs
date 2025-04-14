use entity::framework::ActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::db_controller::DBController;

use super::DBStatus;

impl DBController {
    pub async fn crud_get_or_create_framework(
        &self,
        framework_full_path: &str,
    ) -> Result<DBStatus, DbErr> {
        if let Some(framework) = entity::prelude::Framework::find()
            .filter(entity::framework::Column::FullPath.eq(framework_full_path))
            .one(self.get_connection())
            .await?
        {
            return Ok(DBStatus::AlreadyExists(framework.id));
        }

        let framework = ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            full_path: sea_orm::ActiveValue::Set(framework_full_path.to_owned()),
        };

        let res = framework.insert(self.get_connection()).await?;

        Ok(DBStatus::Created(res.id))
    }
}
