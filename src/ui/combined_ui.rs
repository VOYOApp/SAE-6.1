use std::sync::{Arc, Mutex};
use eframe::egui;
use crate::game_logic::GameLogic;
use crate::types::StyledMessage;
use crate::server::server_thread::ServerSettings;

use crate::ui::game_ui::GameUI;
use crate::ui::server_ui::ServerUi;

pub struct CombinedUI {
    server_ui: ServerUi,
    game_ui: GameUI,
    show_server_ui: bool,
}

impl CombinedUI {
    pub fn new(messages: Arc<Mutex<Vec<StyledMessage>>>, settings: Arc<Mutex<ServerSettings>>, game_logic: Arc<Mutex<GameLogic>>) -> Self {
        CombinedUI {
            server_ui: ServerUi::new(messages.clone(), settings.clone()),
            game_ui: GameUI::new(game_logic), // 💡 à implémenter si besoin
            show_server_ui: true,
        }
    }
}

impl eframe::App for CombinedUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.show_server_ui, "Server UI").clicked() {
                    self.show_server_ui = true;
                }
                if ui.selectable_label(!self.show_server_ui, "Game UI").clicked() {
                    self.show_server_ui = false;
                }
            });
        });

        if self.show_server_ui {
            self.server_ui.update(ctx, frame);
        } else {
            self.game_ui.update(ctx, frame);
        }
    }
}