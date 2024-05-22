#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_extras::{Column, TableBuilder};
mod server_thread;
use server_thread::ServerThread;

fn main() -> Result<(), eframe::Error> {
    let server = ServerThread::new(8080);
    server.start();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Universal Rust Server Software",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::exact(200.0).resizable(false))
                .column(Column::exact(100.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Player Name");
                    });
                    header.col(|ui| {
                        ui.heading("Score");
                    });
                })
                .body(|mut body| {
                    let players = vec!["Lopi5555", "Hethan_hdb", "Vavaaaaaah"];
                    let scores = vec!["5", "19", "0"];

                    for (index, (player, score)) in players.iter().zip(scores.iter()).enumerate() {
                        body.row(30.0, |mut row| {
                            let bg_color = if index % 2 == 0 {
                                egui::Color32::from_gray(20) // Light gray for even rows
                            } else {
                                egui::Color32::from_gray(24) // Almost white for odd rows
                            };
                            row.col(|ui| {
                                ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                ui.label(*player);
                            });
                            row.col(|ui| {
                                ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);
                                ui.label(*score);
                            });
                        });
                    }
                });
        });
    }
}

