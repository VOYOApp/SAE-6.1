use crate::app_defines::AppDefines;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::server::server_thread::ServerSettings;
use crate::types::{add_message, MessageType, StyledMessage};

pub(crate) struct ClientHandler {
    pub(crate) socket: TcpStream,
    pub(crate) buf_writer: BufWriter<TcpStream>,
    pub(crate) buf_reader: BufReader<TcpStream>,
    pub(crate) previous_time: u64,
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
    pub(crate) settings: Arc<Mutex<ServerSettings>>,
}

impl ClientHandler {
    pub fn new(socket: TcpStream, messages: Arc<Mutex<Vec<StyledMessage>>>, settings: Arc<Mutex<ServerSettings>>) -> Self {
        let buf_writer = BufWriter::new(socket.try_clone().unwrap());
        let buf_reader = BufReader::new(socket.try_clone().unwrap());
        ClientHandler {
            socket,
            buf_writer,
            buf_reader,
            previous_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            messages,
            settings,
        }
    }
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

    fn check_timeout(&mut self) -> bool {
        let now = SystemTime::now();
        let current_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time - self.previous_time > AppDefines::CONNECTION_TIMEOUT_DELAY as u64 {
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

    fn process_message(&mut self, received: &str) {
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
    }
    fn handle_disconnection(&mut self) {
        add_message(
            &self.messages,
            format!("[INFO] Client disconnected: {:?}", Result::unwrap(self.socket.peer_addr())),
            MessageType::Info,
        );
        self.socket.shutdown(Shutdown::Both).expect("Failed to shutdown socket");
    }

    pub fn add_to_reponse(mut reponse: String, message: String) {
        if reponse != "" {
            reponse += AppDefines::COMMAND_SEP;
            return reponse += &*message;
        }
    }

    // pub fn make_name_list(){
    // }

    // pub fn leave_game() -> bool {
    //     // CONNECTION_COUNT -= 1;
    //     // TODO ESSAYE D'ENLEVER UN BOT SUR LA MAP
    //     return true;
    // }

    // pub fn join_game() -> bool {
    //     // CONNECTION_COUNT += 1;
    //     // TODO ESSAYE D'AJOUTET UN BOT SUR LA MAP
    //     return true;
    // }

    // pub fn setBot(bot){
    // }

    // pub fn make_default_name() -> String {
    //     return "Player ".to_string() + &*CONNECTION_COUNT.to_string();
    // }

    // pub fn is_offline() -> bool {
    // }
}
