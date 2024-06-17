use std::sync::{Arc, Mutex};
use eframe::egui;

use eframe::egui::{CentralPanel, Context, RichText, TopBottomPanel, Window};
use crate::app_defines::AppDefines;
use crate::StyledMessage;

pub struct ServerUi {
    messages: Arc<Mutex<Vec<StyledMessage>>>,
    show_about: bool,
    show_options: bool,
    arena_width: f32,
    arena_height: f32,
    obstacle_probability: f64,
    game_modes: [&'static str; 1],
    bot_rate_of_fire: i32,
    penalty_time: i64,
    connection_timeout_delay: i32,
    message_duration: i32,
    message_length: i32,
    score_limit: i32,
}

impl ServerUi {
    pub fn new(messages: Arc<Mutex<Vec<StyledMessage>>>) -> Self {
        ServerUi { messages, show_about: false, show_options: false,
            arena_width: AppDefines::ARENA_WIDTH,
            arena_height: AppDefines::ARENA_HEIGHT,
            obstacle_probability: AppDefines::OBSTACLE_PROBABILITY,
            game_modes: AppDefines::GAME_MODES,
            bot_rate_of_fire: AppDefines::BOT_RATE_OF_FIRE,
            penalty_time: AppDefines::PENALTY_TIME,
            connection_timeout_delay: AppDefines::CONNECTION_TIMEOUT_DELAY,
            message_duration: AppDefines::MESSAGE_DURATION,
            message_length: AppDefines::MESSAGE_LENGTH,
            score_limit: AppDefines::SCORE_LIMIT, }
    }

    fn show_menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("General", |ui| {
                    if ui.button("Options").clicked() {
                        self.show_options = true;
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        // Handle exit click
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Afficher la boîte de dialogue About
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn show_about_dialog(&mut self, ctx: &Context) {
        if self.show_about {
            Window::new("About")
                .open(&mut self.show_about)
                .show(ctx, |ui| {
                    ui.label("Android game server");
                    ui.label("Copyright (C) 2024 By BOURGUIGNEAU Ethan");
                    ui.label("CHAVANEL Yohann & SAPET Alan & GRAILLE Théo");
                    ui.label("GNU GENERAL PUBLIC LICENSE");
                    ui.label("Version 1, 30 May 2024");
                    ui.label("");
                    ui.label("This program comes with ABSOLUTELY NO WARRANTY");
                    ui.label("This is free software, and you are welcome to redistribute it");
                    ui.label("under certain conditions");
                });
        }
    }
    fn show_options_dialog(&mut self, ctx: &Context) {
        let mut show_options = self.show_options;
        Window::new("Game Settings")
            .open(&mut show_options)
            .show(ctx, |ui| {

                ui.horizontal(|ui| {
                    ui.label("Connection Timeout Delay:");
                    ui.add(egui::DragValue::new(&mut self.connection_timeout_delay));
                });

                ui.horizontal(|ui| {
                    ui.label("Bot Rate of Fire:");
                    ui.add(egui::DragValue::new(&mut self.bot_rate_of_fire));
                });

                ui.horizontal(|ui| {
                    ui.label("Penalty Time:");
                    ui.add(egui::DragValue::new(&mut self.penalty_time));
                });


                ui.horizontal(|ui| {
                    ui.label("Message Duration:");
                    ui.add(egui::DragValue::new(&mut self.message_duration));
                });

                ui.horizontal(|ui| {
                    ui.label("Message Length:");
                    ui.add(egui::DragValue::new(&mut self.message_length));
                });

                ui.horizontal(|ui| {
                    ui.label("Score Limit:");
                    ui.add(egui::DragValue::new(&mut self.score_limit));
                });

                ui.horizontal(|ui| {
                    ui.label("Arena Width:");
                    ui.add(egui::DragValue::new(&mut self.arena_width));
                });

                ui.horizontal(|ui| {
                    ui.label("Arena Height:");
                    ui.add(egui::DragValue::new(&mut self.arena_height));
                });

                ui.horizontal(|ui| {
                    ui.label("Obstacle Probability:");
                    ui.add(egui::DragValue::new(&mut self.obstacle_probability));
                });

                if ui.button("Apply").clicked() {
                    // Apply changes to server settings here
                    self.show_options = false;
                }
            });
        self.show_options = show_options;
    }
}

impl eframe::App for ServerUi {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu(ctx);
        self.show_about_dialog(ctx);
        self.show_options_dialog(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Server Messages");

            let messages = self.messages.lock().unwrap();
            for message in messages.iter() {
                ui.label(RichText::new(&message.text).color(message.color));
            }
        });
    }
}



