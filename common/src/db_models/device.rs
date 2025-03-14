use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, PartialEq, Eq, Hash, Clone, ToSchema)]
pub struct Device {
    pub id: i32,
    pub model: String,
    pub display_name: Option<String>,
}

impl From<entity::device::Model> for Device {
    fn from(value: entity::device::Model) -> Self {
        Self {
            id: value.id,
            model: value.model,
            display_name: value.display_name,
        }
    }
}
