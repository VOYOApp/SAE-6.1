use std::sync::{Arc, Mutex};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use egui::Key::A;
use crate::app_defines::AppDefines;

use crate::server::client_handler::ClientHandler;
use crate::types::{add_message, MessageType, StyledMessage};

#[derive(Debug)]
pub(crate) struct ServerSettings {
    pub arena_width: f32,
    pub arena_height: f32,
    pub obstacle_probability: f64,
    pub game_modes: Vec<&'static str>,
    pub bot_rate_of_fire: i32,
    pub penalty_time: i64,
    pub connection_timeout_delay: i32,
    pub message_duration: i32,
    pub message_length: i32,
    pub score_limit: i32,
}

impl ServerSettings {
    pub fn new() -> Self {
        ServerSettings {
            arena_width: AppDefines::ARENA_WIDTH,
            arena_height: AppDefines::ARENA_HEIGHT,
            obstacle_probability: AppDefines::OBSTACLE_PROBABILITY,
            game_modes: AppDefines::GAME_MODES.to_vec(),
            bot_rate_of_fire: AppDefines::BOT_RATE_OF_FIRE,
            penalty_time: AppDefines::PENALTY_TIME,
            connection_timeout_delay: AppDefines::CONNECTION_TIMEOUT_DELAY,
            message_duration: AppDefines::MESSAGE_DURATION,
            message_length: AppDefines::MESSAGE_LENGTH,
            score_limit: AppDefines::SCORE_LIMIT,
        }
    }
}

pub(crate) struct ServerThread {
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
    pub(crate) settings: Arc<Mutex<ServerSettings>>,
}

impl ServerThread {
    pub fn new(address: String, port: u16, messages: Arc<Mutex<Vec<StyledMessage>>>, settings: Arc<Mutex<ServerSettings>>) -> Self {
        ServerThread {
            address,
            port,
            messages,
            settings,
        }
    }

    pub(crate) fn start(&self) {
        let listener = TcpListener::bind((self.address.to_string(), self.port)).expect("Could not bind to port");

        add_message(
            &self.messages,
            format!("\n[START] Server address: {:?}", listener.local_addr().unwrap()),
            MessageType::Default,
        );
        add_message(
            &self.messages,
            format!("[START] Listening on port: {}", self.port),
            MessageType::Default,
        );

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    add_message(
                        &self.messages,
                        format!("[INFO] New client connected: {}", stream.peer_addr().unwrap()),
                        MessageType::Info,
                    );
                    let messages = Arc::clone(&self.messages);
                    let settings = Arc::clone(&self.settings);
                    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap(); // Set timeout
                    thread::spawn(move || {
                        ClientHandler::new(stream, messages, settings).run();
                    });
                }
                Err(e) => {
                    add_message(
                        &self.messages,
                        format!("[ERROR] Connection failed: {}", e),
                        MessageType::Error,
                    );
                }
            }
        }
    }
}
