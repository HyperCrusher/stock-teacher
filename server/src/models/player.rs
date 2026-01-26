use crate::db::get_uuid;
use uuid::Uuid;

#[derive(Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            id: get_uuid(),
            name: name.to_string(),
        }
    }
}
