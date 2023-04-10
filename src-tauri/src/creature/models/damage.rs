use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Damage {
    id: String,
    damage: String,
    damage_dice: String,
    damage_type: String,
}
