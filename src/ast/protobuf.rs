use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumField {
    pub id: isize,
    pub name: String,
    pub comment: Option<String>,
}
