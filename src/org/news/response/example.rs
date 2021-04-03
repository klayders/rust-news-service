use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct News {
    pub id: u32,
    pub title: String,
}