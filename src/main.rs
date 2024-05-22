mod server;
mod ui;

use eframe::egui;
use crate::server::server_thread::ServerThread;
use crate::ui::game_ui::MyApp;

use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Lancer l'interface utilisateur dans un thread séparé
    let ui_thread = thread::spawn(|| {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_min_inner_size([1100.0, 700.0]),
            ..Default::default()
        };
        eframe::run_native(
            "Universal Rust Server Software",
            options,
            Box::new(|_| Box::<MyApp>::default()),
        );
    });

    // Lancer le serveur dans le thread principal
    let serv = ServerThread::new(6969);
    serv.start();

    // Attendre que le thread de l'interface utilisateur se termine (ce qui ne se produira probablement jamais)
    ui_thread.join().expect("UI thread panicked!");

    Ok(())
}


