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
    ClientExit,
    ClientDisconnect,
    Default,
}

impl MessageType {
    pub fn to_color(&self) -> egui::Color32 {
        match self {
            MessageType::Info => egui::Color32::from_rgb(0, 255, 0),         // Green
            MessageType::Error => egui::Color32::from_rgb(255, 0, 0),       // Red
            MessageType::Warning => egui::Color32::from_rgb(255, 255, 0),   // Yellow
            MessageType::ClientExit => egui::Color32::from_rgb(0, 0, 255),  // Blue
            MessageType::ClientDisconnect => egui::Color32::from_rgb(255, 165, 0), // Orange
            MessageType::Default => egui::Color32::from_rgb(0, 0, 0),       // Black
        }
    }
}

