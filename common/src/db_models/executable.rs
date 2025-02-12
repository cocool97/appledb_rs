use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Executable {
    pub id: i32,
    pub name: String,
    pub operating_system_version_id: i32,
}

impl From<entity::executable::Model> for Executable {
    fn from(value: entity::executable::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            operating_system_version_id: value.operating_system_version_id,
        }
    }
}
