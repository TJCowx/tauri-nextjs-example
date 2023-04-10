use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    id: String,
    name: String,
    description: String,
}
