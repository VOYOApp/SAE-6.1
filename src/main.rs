use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub use types::StyledMessage;
use crate::game_logic::GameLogic;
use crate::server::server_thread::{ServerSettings, ServerThread};
use crate::ui::CombinedUI;  // <-- Import de ta nouvelle UI combinée

mod server;
mod ui;
mod app_defines;
pub mod types;
mod physics;
mod entities;
mod bullet;
mod game_logic;
mod obstacles;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let settings = Arc::new(Mutex::new(ServerSettings::new()));
    let game_logic = Arc::new(Mutex::new(GameLogic::new())); // ✅ ici

    let server_messages = Arc::clone(&messages);
    let server_settings = Arc::clone(&settings);
    let server_game_logic = Arc::clone(&game_logic); // ✅

    thread::spawn(move || {
        let serv = ServerThread {
            address: "127.0.0.1".to_string(),
            port: 6969,
            messages: server_messages,
            settings: server_settings,
            game_logic: server_game_logic, // ✅ partagé
            client_entity_map: Arc::new(Mutex::new(HashMap::new())),
        };
        serv.start();
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics Simulation & Server GUI",
        native_options,
        Box::new(|_cc| Box::new(CombinedUI::new(messages, settings, game_logic))), // ✅ ici aussi
    )?;

    Ok(())
}
