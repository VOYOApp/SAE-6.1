use std::sync::{Arc, Mutex};
use std::thread;

use bevy::prelude::*;

mod server;
use server::Entity::Entity;
use server::game_management::{block_players_in_bound, player_mov, resize_boundaries, spawn_boundaries, spawn_camera, spawn_player};
pub use types::StyledMessage;

use crate::server::server_thread::{ServerSettings, ServerThread};
use crate::ui::game_ui::GameUI;

mod ui;
mod app_defines;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //  STARTING UI AND MOVEMENTS
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_camera, spawn_player, spawn_boundaries).chain())
    .add_systems(Update, player_mov)
    .add_systems(Update, block_players_in_bound.after(player_mov))
    .add_systems(Update, resize_boundaries)
    .run();

    // Shared state for messages
    let messages = Arc::new(Mutex::new(Vec::new()));
    let settings = Arc::new(Mutex::new(ServerSettings::new()));

    // Clone the Arcs to move into the server thread
    let server_messages = Arc::clone(&messages);
    let server_settings = Arc::clone(&settings);

    // Start the server in a separate thread
    thread::spawn(move || {
        let serv = ServerThread::new(6969, server_messages, server_settings);
        serv.start();
    });

    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Physics Simulation",
    //     options,
    //     Box::new(|_cc| Box::new(GameUI::default())),
    // ).expect("TODO: panic message");

    // Run the GUI in the main thread
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Server GUI",
        native_options,
        Box::new(|_cc| Box::new(ui::server_ui::ServerUi::new(messages, settings))),
    ).expect("Failed to run server GUI");

    Ok(())

}