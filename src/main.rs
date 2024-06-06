use std::sync::{Arc, Mutex};
use std::thread;
use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::winit::WinitSettings;

pub use types::StyledMessage;

use crate::server::server_thread::ServerThread;

mod server;
mod ui;
mod app_defines;
pub mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Shared state for messages
    let messages = Arc::new(Mutex::new(Vec::new()));

    // Clone the Arc to move into the server thread
    let server_messages = Arc::clone(&messages);

    // Start the server in a separate thread
    thread::spawn(move || {
        let serv = ServerThread::new(6969, server_messages);
        serv.start();
    });

    let options = eframe::NativeOptions::default();
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, crate::ui::game_ui::setup)
        .run();

    // Run the GUI in the main thread
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Server GUI",
        native_options,
        Box::new(|_cc| Box::new(ui::server_ui::ServerUi::new(messages))),
    ).expect("Failed to run server GUI");

    Ok(())
}

