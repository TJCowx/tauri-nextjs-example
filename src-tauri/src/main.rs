#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod creature;
mod mongo;

pub use crate::creature::*;
pub use crate::mongo::connect_db;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            connect_db,
            creature::queries::get_all_creatures,
            creature::queries::add_creature,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
