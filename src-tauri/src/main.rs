// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parser;
mod config;

use parser::{players::{self, Players}, ParserError};
use serde::{Deserialize, Serialize};
use config::Config;

#[derive(Deserialize, Serialize)]
struct AnyItem {
    #[serde(rename = "@attribute")]
    attribute: String,

    #[serde(rename = "$value")]
    value: String,
}

fn load_config() -> Config {
    config::load_config("/home/fert/projects/ittf-merger/merger.yaml".into())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn generate_excel_file<'a>(config: tauri::State<'a, Config>) -> Result<Players, ParserError> {
    let players = players::parse_players(config.path());

    // return Ok(result.unwrap());
    return players;
}

fn main() {
    tauri::Builder::default()
        .manage(load_config())
        .invoke_handler(tauri::generate_handler![generate_excel_file])
        .run(tauri::generate_context!())
        .expect("Error while starting application");
}
