use crate::mongo::*;
use futures::stream::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Creature {
    id: String,
    name: String,
    size: String,
    creature_type: String,
    alignment: String,
    armour_class: i32,
    hit_points: String,
    hit_die: String,
    land_speed: i32,
    fly_speed: i32,
    burrow_speed: i32,
    climb_speed: i32,
    hover_speed: i32,
    blindsight: i32,
    darkvision: i32,
    tremorsense: i32,
    truesight: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    prof_bonus: i32,
    // TODO: Proficiencies
    // TODO: Saving Throws
    // TODO: Immunities
    // TODO: Condition Immunities
    // TODO: Resistances
    // TODO: Weaknesses
    // TODO: Languages
    // TODO: Actions
    // TODO: Ability
    is_legendary: bool,
    has_lair: bool,
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

    // TODO: Handle this properly
    let res = collection
        .insert_one(creature, None)
        .await
        .map_err(|e| e.to_string());

    push_connection(client);

    Ok(())
}
