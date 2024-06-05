use std::sync::{Arc, Mutex};
use eframe::egui::{self, CentralPanel, Context, RichText, TopBottomPanel, Window};
use crate::ServerSettings;
use crate::StyledMessage;

pub struct ServerUi {
    messages: Arc<Mutex<Vec<StyledMessage>>>,
    settings: Arc<Mutex<ServerSettings>>,
    show_about: bool,
    show_options: bool,
}

impl ServerUi {
    pub fn new(messages: Arc<Mutex<Vec<StyledMessage>>>, settings: Arc<Mutex<ServerSettings>>) -> Self {
        ServerUi {
            messages,
            settings,
            show_about: false,
            show_options: false,
        }
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
        let mut settings = self.settings.lock().unwrap();
        Window::new("Game Settings")
            .open(&mut show_options)
            .show(ctx, |ui| {

                ui.horizontal(|ui| {
                    ui.label("Connection Timeout Delay:");
                    ui.add(egui::DragValue::new(&mut settings.connection_timeout_delay));
                });

                ui.horizontal(|ui| {
                    ui.label("Bot Rate of Fire:");
                    ui.add(egui::DragValue::new(&mut settings.bot_rate_of_fire));
                });

                ui.horizontal(|ui| {
                    ui.label("Penalty Time:");
                    ui.add(egui::DragValue::new(&mut settings.penalty_time));
                });


                ui.horizontal(|ui| {
                    ui.label("Message Duration:");
                    ui.add(egui::DragValue::new(&mut settings.message_duration));
                });

                ui.horizontal(|ui| {
                    ui.label("Message Length:");
                    ui.add(egui::DragValue::new(&mut settings.message_length));
                });

                ui.horizontal(|ui| {
                    ui.label("Score Limit:");
                    ui.add(egui::DragValue::new(&mut settings.score_limit));
                });

                ui.horizontal(|ui| {
                    ui.label("Arena Width:");
                    ui.add(egui::DragValue::new(&mut settings.arena_width));
                });

                ui.horizontal(|ui| {
                    ui.label("Arena Height:");
                    ui.add(egui::DragValue::new(&mut settings.arena_height));
                });

                ui.horizontal(|ui| {
                    ui.label("Obstacle Probability:");
                    ui.add(egui::DragValue::new(&mut settings.obstacle_probability));
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



