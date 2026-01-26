use crate::db::get_uuid;
use crate::{auth, errors};
use uuid::Uuid;

pub struct Game {
    pub id: Uuid,
    pub gm: Uuid,
    pub password: Option<String>,
    pub active: bool,
}

impl Game {
    pub fn new(gm: &Uuid, password: Option<&str>) -> Result<Self, errors::StError> {
        let pass = match password {
            Some(pass) => Some(auth::hash(pass)?),
            None => None,
        };
        Ok(Self {
            id: get_uuid(),
            gm: gm.clone(),
            password: pass,
            active: true,
        })
    }
}
