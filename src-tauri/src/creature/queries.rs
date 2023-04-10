use crate::models::{creature::Creature, new_creature::NewCreature};
use crate::mongo::*;
use futures::stream::TryStreamExt;

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
pub async fn add_creature(creature: NewCreature) -> Result<(), String> {
    let client = get_connection();

    let collection = client
        .database("5e-dm-tools")
        .collection::<NewCreature>("creatures");

    println!("[server] Adding creature {}", creature.name);

    // TODO: Check for creature of same name and conflict if there is one.

    // TODO: Handle this properly, explore the response.
    let res = collection
        .insert_one(creature, None)
        .await
        .map_err(|e| e.to_string());

    println!("[server] Creature successfully added!");

    push_connection(client);

    Ok(())
}
