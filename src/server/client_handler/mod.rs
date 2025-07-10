use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app_defines::AppDefines;
use crate::game_logic::GameLogic;
use crate::server::server_thread::ServerSettings;
use crate::types::{add_message, MessageType, StyledMessage};

/// A struct representing a client handler, responsible for communicating with a client via a TCP socket.
pub(crate) struct ClientHandler {
    /// The TCP socket associated with the client.
    pub(crate) socket: TcpStream,
    /// A buffer for writing data to the socket.
    pub(crate) buf_writer: BufWriter<TcpStream>,
    /// A buffer for reading data from the socket.
    pub(crate) buf_reader: BufReader<TcpStream>,
    /// The time in seconds since the Unix epoch of the client's last activity.
    pub(crate) previous_time: u64,
    /// A thread-safe, shared vector of styled messages.
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
    /// Thread-safe, shared server settings.
    pub(crate) settings: Arc<Mutex<ServerSettings>>,
    game_logic: Arc<Mutex<GameLogic>>,
    client_entity_map: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl ClientHandler {
    /// Creates a new client handler with the specified socket, messages, and server settings.
    ///
    /// # Arguments
    ///
    /// * `socket` - The client's TCP socket.
    /// * `messages` - A thread-safe, shared vector of styled messages.
    /// * `settings` - Thread-safe, shared server settings.
    ///
    /// # Returns
    ///
    /// A new `ClientHandler`.
    ///
    pub fn new(socket: TcpStream,
               messages: Arc<Mutex<Vec<StyledMessage>>>,
               settings: Arc<Mutex<ServerSettings>>,
               game_logic: Arc<Mutex<GameLogic>>,
               client_entity_map: Arc<Mutex<HashMap<SocketAddr, u32>>>,
        ) -> Self {
        let buf_writer = BufWriter::new(socket.try_clone().unwrap());
        let buf_reader = BufReader::new(socket.try_clone().unwrap());
        ClientHandler {
            socket,
            buf_writer,
            buf_reader,
            previous_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            messages,
            settings,
            game_logic,
            client_entity_map,
        }
    }

    /// Starts the client handler, reading messages from the client and processing them until disconnection or timeout.
    pub fn run(&mut self) {
        let mut received_message = String::new();
        let mut running = true;
        while running {
            if self.check_timeout() {
                break;
            }

            if let Ok(message_length) = self.buf_reader.read_line(&mut received_message) {
                if message_length > 1 {
                    self.handle_received_message(&received_message);
                    received_message.clear();
                } else {
                    self.handle_disconnection();
                    running = false;
                    break;
                }
            }
        }
    }

    /// Checks if the client has exceeded the inactivity timeout.
    ///
    /// # Returns
    ///
    /// `true` if the client has exceeded the inactivity timeout, `false` otherwise.
    ///
    fn check_timeout(&mut self) -> bool {
        let now = SystemTime::now();
        let current_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time - self.previous_time > AppDefines::CONNECTION_TIMEOUT_DELAY as u64 {
            let peer_addr = self.socket.peer_addr().unwrap();

            if let Some(entity_id) = self.client_entity_map.lock().unwrap().remove(&peer_addr) {
                let mut logic = self.game_logic.lock().unwrap();
                logic.remove_entity_by_id(entity_id);
            }

            add_message(
                &self.messages,
                format!("[WARNING] Connection timeout: {}", self.socket.peer_addr().unwrap()),
                MessageType::Warning,
            );
            self.socket.shutdown(std::net::Shutdown::Both).unwrap();
            true
        } else {
            false
        }
    }

    /// Handles a message received from the client.
    ///
    /// # Arguments
    ///
    /// * `received_message` - The received message as a string.
    ///
    fn handle_received_message(&mut self, received_message: &str) {
        let all_messages: Vec<&str> = received_message.trim().split(AppDefines::COMMAND_SEP).collect();
        for message in all_messages {
            // println!("Message {:?}", message);
            match message {
                AppDefines::QUIT => {
                    self.handle_disconnection();
                    return;
                }
                _ => self.process_message(message),
            };
            self.previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    }

    /// Processes an individual message from the client.
    ///
    /// # Arguments
    ///
    /// * `received` - The received message as a string.
    ///
   /* fn process_message(&mut self, received: &str) {
        let mut message_values = received.split(AppDefines::ARGUMENT_SEP).collect::<Vec<&str>>();
        let code_message = message_values[0];
        println!("Processing message: {:?}", received);
        println!("Values: {:?}", message_values);
        println!("Commande values: {:?}", code_message);
        println!("\n");
        let response = match code_message {
            AppDefines::SET_NAME => {
                "TODO SET NAME".to_string()
            }
            AppDefines::SET_COLOR => {
                "TODO SET COLOR".to_string()
            }
            AppDefines::ALIVE => {
                "TODO ALIVE".to_string()
            }
            AppDefines::MESSAGE => {
                "TODO MESSAGE".to_string()
            }
            AppDefines::QUERY_CLOSEST_BOT => {
                "TODO QUERY CLOSEST BOT".to_string()
            }
            AppDefines::QUERY_CLOSEST_PROJECTILE => {
                "TODO QUERY CLOSEST PROJECTILE".to_string()
            }
            AppDefines::QUERY_BY_NAME => {
                "TODO QUERY BY NAME".to_string()
            }
            AppDefines::QUERY_NAME_LIST => {
                "TODO QUERY NAME LIST".to_string()
            }
            AppDefines::QUERY_ORIENTATION => {
                "TODO QUERY ORIENTATION".to_string()
            }
            AppDefines::QUERY_MESSAGES_FROM_USER => {
                "TODO QUERY MESSAGES FROM USER".to_string()
            }
            AppDefines::EMPTY_REPLY => {
                "TODO EMPTY REPLY".to_string()
            }
            _ => {
                "ERROR".to_string()
            }
        };
        if let Err(e) = self.buf_writer.write_all(response.as_bytes()) {
            println!("Failed to send response: {}", e);
        }

        if let Err(e) = self.buf_writer.flush() {
            println!("Failed to flush response: {}", e);
        }
    }*/

    /// Processes an individual message from the client.
    ///
    /// # Arguments
    ///
    /// * `received` - The received message as a string.
    ///
    fn process_message(&mut self, received: &str) {
        let peer_addr = self.socket.peer_addr().unwrap();
        let entity_id = {
            let map = self.client_entity_map.lock().unwrap();
            *map.get(&peer_addr).unwrap_or(&0)
        };

        let mut parts = received.trim().split(AppDefines::ARGUMENT_SEP);
        let code = parts.next().unwrap_or("").trim();
        let value = parts.next();

        let response = match code {
            AppDefines::SET_NAME => {
                if let Some(name) = value {
                    let mut logic = self.game_logic.lock().unwrap();
                    if let Some(entity) = logic.get_entity_mut(entity_id) {
                        entity.set_name(name.to_string());
                        format!("Name set to {}", name)
                    } else {
                        "Entity not found".to_string()
                    }
                } else {
                    "Missing name".to_string()
                }
            }

            AppDefines::ACTUATOR_MOTOR_LEFT |
            AppDefines::ACTUATOR_MOTOR_RIGHT |
            AppDefines::ACTUATOR_GUN_TRIGGER |
            AppDefines::ACTUATOR_GUN_TRAVERSE => {
                if let Some(val_str) = value {
                    match val_str.trim().parse::<f32>() {
                        Ok(val) => {
                            let mut logic = self.game_logic.lock().unwrap();
                            if let Some(ent) = logic.get_entity_mut(entity_id) {
                                match code {
                                    AppDefines::ACTUATOR_MOTOR_LEFT => ent.motor_left = val,
                                    AppDefines::ACTUATOR_MOTOR_RIGHT => ent.motor_right = val,
                                    AppDefines::ACTUATOR_GUN_TRIGGER => ent.gun_trigger = val,
                                    AppDefines::ACTUATOR_GUN_TRAVERSE => ent.gun_traverse = val,
                                    _ => {}
                                }
                                format!("{} set to {}", code, val)
                            } else {
                                "Entity not found".to_string()
                            }
                        }
                        Err(_) => "Invalid float value".to_string(),
                    }
                } else {
                    "Missing value".to_string()
                }
            }

            AppDefines::QUIT => {
                self.handle_disconnection();
                return; // on ne continue pas après EXIT
            }

            _ => format!("Unknown command: {}", code),
        };

        let _ = writeln!(self.buf_writer, "{}", response);
        let _ = self.buf_writer.flush();
    }


    fn handle_disconnection(&mut self) {
        let peer_addr = self.socket.peer_addr().unwrap();

        if let Some(entity_id) = self.client_entity_map.lock().unwrap().remove(&peer_addr) {
            let mut logic = self.game_logic.lock().unwrap();
            logic.remove_entity_by_id(entity_id);
            add_message(
                &self.messages,
                format!("[INFO] Client {} disconnected, entity {} removed.", peer_addr, entity_id),
                MessageType::Info,
            );
        } else {
            add_message(
                &self.messages,
                format!("[INFO] Client {} disconnected, but had no associated entity.", peer_addr),
                MessageType::Info,
            );
        }

        let _ = self.socket.shutdown(Shutdown::Both);
    }

    /// Adds a message to the response string.
    ///
    /// # Arguments
    ///
    /// * `response` - The existing response string.
    /// * `message` - The message to add to the response.
    ///
    /// # Returns
    ///
    /// The updated response string with the new message appended.
    ///
    pub fn add_to_reponse(mut reponse: String, message: String) {
        if reponse != "" {
            reponse += AppDefines::COMMAND_SEP;
            return reponse += &*message;
        }
    }
}
