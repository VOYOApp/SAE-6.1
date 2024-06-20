use std::sync::{Arc, Mutex};
use eframe::egui;

/// Struct representing a styled message with text and color.
pub struct StyledMessage {
    /// The text of the message.
    pub text: String,
    /// The color of the message.
    pub color: egui::Color32,
}

impl StyledMessage {
    /// Creates a new styled message.
    ///
    /// # Arguments
    ///
    /// * `text` - The text of the message.
    /// * `message_type` - The type of the message which determines its color.
    ///
    /// # Returns
    ///
    /// A new `StyledMessage` with the specified text and color.
    pub fn new(text: String, message_type: MessageType) -> Self {
        StyledMessage {
            text,
            color: message_type.to_color(),
        }
    }
}

/// Enum representing different types of messages.
pub enum MessageType {
    Info,
    Error,
    Warning,
    Debug,
    Default,
}

impl MessageType {
    /// Converts a message type to a corresponding color.
    ///
    /// # Returns
    ///
    /// The color associated with the message type.
    pub fn to_color(&self) -> egui::Color32 {
        match self {
            MessageType::Info => egui::Color32::GREEN,      // Green for info messages
            MessageType::Error => egui::Color32::RED,       // Red for error messages
            MessageType::Warning => egui::Color32::YELLOW,  // Yellow for warning messages
            MessageType::Debug => egui::Color32::BLUE,      // Blue for debug messages
            MessageType::Default => egui::Color32::GRAY,    // Gray for default messages
        }
    }
}

/// Adds a new message to the list of messages.
///
/// # Arguments
///
/// * `messages` - A thread-safe reference to the list of messages.
/// * `text` - The text of the message.
/// * `message_type` - The type of the message which determines its color.
///
/// This function locks the list of messages, creates a new styled message, and adds it to the list.
pub fn add_message(messages: &Arc<Mutex<Vec<StyledMessage>>>, text: String, message_type: MessageType) {
    let message = StyledMessage::new(text, message_type);
    messages.lock().unwrap().push(message);
}

