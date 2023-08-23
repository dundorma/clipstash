use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}
impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::from_str(s) {
            Ok(id) => Ok(id.into()),
            Err(e) => Err(e),
        }
    }
}
