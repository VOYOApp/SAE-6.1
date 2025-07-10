use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::app_defines::AppDefines;
use crate::entities::entity::Entity;
use crate::game_logic::GameLogic;
use crate::server::client_handler::ClientHandler;
use crate::types::{add_message, MessageType, StyledMessage};

/// A struct representing server settings.
#[derive(Debug)]
pub(crate) struct ServerSettings {
    /// The width of the arena.
    pub arena_width: f32,
    /// The height of the arena.
    pub arena_height: f32,
    /// The probability of obstacles appearing in the arena.
    pub obstacle_probability: f64,
    /// The available game modes.
    pub game_modes: Vec<&'static str>,
    /// The rate of fire for bots.
    pub bot_rate_of_fire: i32,
    /// The penalty time for infractions.
    pub penalty_time: i64,
    /// The delay before a connection times out.
    pub connection_timeout_delay: i32,
    /// The duration messages are displayed.
    pub message_duration: i32,
    /// The maximum length of a message.
    pub message_length: i32,
    /// The score limit for the game.
    pub score_limit: i32,
}

impl ServerSettings {
    /// Creates a new instance of `ServerSettings` with default values from `AppDefines`.
    ///
    /// # Returns
    ///
    /// A new `ServerSettings` instance.
    ///
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

/// A struct representing a server thread.
pub(crate) struct ServerThread {
    /// The address on which the server listens.
    pub(crate) address: String,
    /// The port on which the server listens.
    pub(crate) port: u16,
    /// A thread-safe, shared vector of styled messages.
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
    /// Thread-safe, shared server settings.
    pub(crate) settings: Arc<Mutex<ServerSettings>>,
    /// Game logic shared with the simulation
    pub(crate) game_logic: Arc<Mutex<GameLogic>>,
    /// Map client -> entity
    pub(crate) client_entity_map: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl ServerThread {
    /// Creates a new server thread with the specified address, port, messages, and settings.
    ///
    /// # Arguments
    ///
    /// * `address` - The address on which the server listens.
    /// * `port` - The port on which the server listens.
    /// * `messages` - A thread-safe, shared vector of styled messages.
    /// * `settings` - Thread-safe, shared server settings.
    ///
    /// # Returns
    ///
    /// A new `ServerThread` instance.
    ///
    pub fn new(address: String, port: u16, messages: Arc<Mutex<Vec<StyledMessage>>>, settings: Arc<Mutex<ServerSettings>>) -> Self {
        ServerThread {
            address,
            port,
            messages,
            settings,
            game_logic: Arc::new(Mutex::new(GameLogic::new())),
            client_entity_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Starts the server thread, listening for incoming connections and spawning a new client handler for each connection.
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
                    let peer_addr = stream.peer_addr().unwrap();

                    add_message(
                        &self.messages,
                        format!("[INFO] New client connected: {}", peer_addr),
                        MessageType::Info,
                    );

                    // Création de l'entité pour le client
                    let entity_id = {
                        let mut logic = self.game_logic.lock().unwrap();
                        logic.add_entity("Player".to_string())
                    };

                    // Sauvegarde de l'association client -> entity
                    self.client_entity_map
                        .lock()
                        .unwrap()
                        .insert(peer_addr, entity_id);

                    let messages = Arc::clone(&self.messages);
                    let settings = Arc::clone(&self.settings);
                    let game_logic = Arc::clone(&self.game_logic);
                    let client_map = Arc::clone(&self.client_entity_map);

                    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap(); // Set timeout

                    thread::spawn(move || {
                        ClientHandler::new(stream, messages, settings, game_logic, client_map).run();
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

