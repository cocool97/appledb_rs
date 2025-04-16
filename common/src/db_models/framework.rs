use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Eq, Hash)]
pub struct Framework {
    pub id: i64,
    pub full_path: String,
}

impl From<entity::framework::Model> for Framework {
    fn from(value: entity::framework::Model) -> Self {
        Self {
            id: value.id,
            full_path: value.full_path,
        }
    }
}
