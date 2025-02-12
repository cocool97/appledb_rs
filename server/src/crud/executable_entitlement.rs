use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};

use crate::db_controller::DBController;

impl DBController {
    pub async fn crud_create_executable_entitlement(
        &self,
        executable_id: i32,
        entitlement_id: i32,
    ) -> Result<(), DbErr> {
        let executable_entitlement = entity::executable_entitlement::ActiveModel {
            executable_id: ActiveValue::Set(executable_id),
            entitlement_id: ActiveValue::Set(entitlement_id),
        };

        executable_entitlement.insert(self.get_connection()).await?;

        Ok(())
    }
}
