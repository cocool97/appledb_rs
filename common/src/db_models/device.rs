use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, PartialEq, Eq, Hash, Clone, ToSchema)]
pub struct Device {
    pub id: i32,
    pub model_code: String,
    pub display_name: Option<String>,
}

impl From<entity::device::Model> for Device {
    fn from(value: entity::device::Model) -> Self {
        Self {
            id: value.id,
            model_code: value.model_code,
            display_name: value.display_name,
        }
    }
}
