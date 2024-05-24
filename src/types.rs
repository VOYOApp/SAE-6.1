use std::sync::{Arc, Mutex};
use eframe::egui;

pub struct StyledMessage {
    pub text: String,
    pub color: egui::Color32,
}

impl StyledMessage {
    pub fn new(text: String, message_type: MessageType) -> Self {
        StyledMessage {
            text,
            color: message_type.to_color(),
        }
    }
}

pub enum MessageType {
    Info,
    Error,
    Warning,
    Debug,
    Default,
}

impl MessageType {
    pub fn to_color(&self) -> egui::Color32 {
        match self {
            MessageType::Info => egui::Color32::GREEN,         // Green
            MessageType::Error => egui::Color32::RED,       // Red
            MessageType::Warning => egui::Color32::YELLOW,   // Yellow
            MessageType::Debug => egui::Color32::BLUE,     // Blue
            MessageType::Default => egui::Color32::GRAY,       // Black
        }
    }
}

pub fn add_message(messages: &Arc<Mutex<Vec<StyledMessage>>>, text: String, message_type: MessageType) {
    let message = StyledMessage::new(text, message_type);
    messages.lock().unwrap().push(message);
}

