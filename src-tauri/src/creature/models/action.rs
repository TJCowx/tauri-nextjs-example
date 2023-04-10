use crate::models::damage::Damage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    id: String,
    name: String,
    description: String,
    action_type: String,
    is_attack: bool,
    attack_delivery: Option<String>,
    to_hit: Option<i16>,
    damage: Option<Vec<Damage>>,
    reach: Option<i16>,
}
