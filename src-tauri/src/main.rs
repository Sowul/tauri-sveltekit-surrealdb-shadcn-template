// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
use crate::error::Error;

mod db;
use crate::db::{query, DbCxn};

mod model;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello from Rust, {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let filename = "bases\\test.db";
    let client = DbCxn::init(filename).await?;

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![greet, query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
