use std::io::{BufRead, BufReader, BufWriter};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app_defines::AppDefines;
use crate::types::{add_message, MessageType, StyledMessage};

const CONNECTION_COUNT: u64 = 0;
// const INTERRUPT: bool = false;

pub(crate) struct ClientHandler {
    pub(crate) socket : TcpStream,
    pub(crate) buf_writer: BufWriter<TcpStream>,
    pub(crate) buf_reader: BufReader<TcpStream>,
    pub(crate) previous_time: u64,
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
}

impl ClientHandler {
    pub fn new(socket: TcpStream, messages: Arc<Mutex<Vec<StyledMessage>>>) -> Self {
        let buf_writer = BufWriter::new(socket.try_clone().unwrap());
        let buf_reader = BufReader::new(socket.try_clone().unwrap());
        ClientHandler {
            socket,
            buf_writer,
            buf_reader,
            previous_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            messages,
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
            println!("Message {:?}", message);
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
        println!("Default TODO PROCESS MESSAGE");
    }
    fn handle_disconnection(&mut self) {
        println!("Client disconnected: {:?}", self.socket.peer_addr().unwrap());
        add_message(
            &self.messages,
            format!("[INFO] Client disconnected: {}", Result::unwrap(self.socket.peer_addr().unwrap()))),
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
}