use crate::mongo::*;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

// TODO: Update this to use models specific to queries
#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {
    id: Option<String>,
    name: String,
    size: Option<String>,
    creature_type: Option<String>,
    alignment: Option<String>,
    armour_class: Option<i32>,
    hit_points: Option<String>,
    hit_die: Option<String>,
    land_speed: Option<i32>,
    fly_speed: Option<i32>,
    burrow_speed: Option<i32>,
    climb_speed: Option<i32>,
    hover_speed: Option<i32>,
    blindsight: Option<i32>,
    darkvision: Option<i32>,
    tremorsense: Option<i32>,
    truesight: Option<i32>,
    strength: Option<i32>,
    dexterity: Option<i32>,
    constitution: Option<i32>,
    intelligence: Option<i32>,
    wisdom: Option<i32>,
    charisma: Option<i32>,
    prof_bonus: Option<i32>,
    // TODO: Proficiencies
    // TODO: Saving Throws
    // TODO: Immunities
    // TODO: Condition Immunities
    // TODO: Resistances
    // TODO: Weaknesses
    // TODO: Languages
    // TODO: Actions
    // TODO: Ability
    is_legendary: Option<bool>,
    has_lair: Option<bool>,
}

#[tauri::command]
pub async fn get_all_creatures() -> Result<Vec<Creature>, String> {
    let client = get_connection();

    let db = client.database("5e-dm-tools");

    println!("[server] Retrieving creatures...");

    let mut cursor = db
        .collection::<Creature>("creatures")
        .find(None, None)
        .await
        .map_err(|e| e.to_string())?;

    let mut res = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        res.push(doc);
    }

    push_connection(client);

    println!("[server] Creatures received successfully!");

    Ok(res)
}

#[tauri::command]
pub async fn add_creature(creature: Creature) -> Result<(), String> {
    let client = get_connection();

    let collection = client
        .database("5e-dm-tools")
        .collection::<Creature>("creatures");

    println!("[server] Adding creature {}", creature.name);

    // TODO: Check for creature of same name and conflict if there is one.

    // TODO: Handle this properly
    let res = collection
        .insert_one(creature, None)
        .await
        .map_err(|e| e.to_string());

    println!("[server] Creature successfully added!");

    push_connection(client);

    Ok(())
}
