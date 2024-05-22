mod server;
mod ui;
mod app_defines;

use eframe::egui;
use crate::server::server_thread::ServerThread;
use crate::ui::game_ui::GameUI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([1100.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Universal Rust Server Software",
        options,
        Box::new(|_| {
            Box::<GameUI>::default()
        }),
    ).expect("TODO: panic message");

    let serv = ServerThread::new(6969);
    serv.start();
    Ok(())
}

