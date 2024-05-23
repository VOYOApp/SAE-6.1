use std::sync::{Arc, Mutex};
use eframe::egui::RichText;
use crate::StyledMessage;

pub struct ServerUi {
    messages: Arc<Mutex<Vec<StyledMessage>>>,
}

impl ServerUi {
    pub fn new(messages: Arc<Mutex<Vec<StyledMessage>>>) -> Self {
        ServerUi { messages }
    }
}

impl eframe::App for ServerUi {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Server Messages");

            let messages = self.messages.lock().unwrap();
            for message in messages.iter() {
                ui.label(RichText::new(&message.text).color(message.color));
            }
        });
    }
}
