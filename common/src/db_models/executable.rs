use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone, PartialEq, Eq, Hash)]
pub struct Executable {
    pub id: i32,
    pub name: String,
    pub full_path: String,
}

impl From<entity::executable::Model> for Executable {
    fn from(value: entity::executable::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            full_path: value.full_path,
        }
    }
}
