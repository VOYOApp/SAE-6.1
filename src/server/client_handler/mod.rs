use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui;
use crate::app_defines::AppDefines;
use crate::types::{StyledMessage, MessageType};

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
            let message = StyledMessage::new(
                format!("[INFO] Connection timeout: {}", self.socket.peer_addr().unwrap()),
                MessageType::Error,
            );
            self.messages.lock().unwrap().push(message);
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
            let message_type = match message {
                AppDefines::QUIT => {
                    self.handle_client_exit();
                    return;
                }
                _ => MessageType::Default,
            };
            let styled_message = StyledMessage::new(message.to_string(), message_type);
            self.messages.lock().unwrap().push(styled_message);
            self.previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    }

    fn handle_client_exit(&mut self) {
        let message = StyledMessage::new(
            format!("[INFO] Client exited the game: {}", self.socket.peer_addr().unwrap()),
            MessageType::ClientExit,
        );
        self.messages.lock().unwrap().push(message);
        self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    }

    fn handle_disconnection(&mut self) {
        println!("Client disconnected: {:?}", self.socket.peer_addr().unwrap());
        let message = StyledMessage::new(
            format!("[INFO] Client disconnected: {}", self.socket.peer_addr().unwrap()),
            MessageType::ClientDisconnect,
        );
        self.messages.lock().unwrap().push(message);
        self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    }
    // pub fn run(&mut self) {
    //     let mut received_message = String::new();
    //     let mut running = true;
    //
    //     while running {
    //         let now = SystemTime::now();
    //         let current_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    //
    //         // println!("{:?} {:?} {:?}", current_time, self.previous_time, AppDefines::CONNECTION_TIMEOUT_DELAY as u64);
    //         // println!("{:?}", current_time - self.previous_time);
    //
    //         if current_time - self.previous_time > AppDefines::CONNECTION_TIMEOUT_DELAY as u64 {
    //             println!("Connection timeout {:?}", self.socket.peer_addr().unwrap());
    //             let message = StyledMessage {
    //                 text: format!("[INFO] Connection timeout: {}", self.socket.peer_addr().unwrap()),
    //                 color: egui::Color32::from_rgb(255, 0, 0), // Red color for timeout messages
    //             };
    //             self.messages.lock().unwrap().push(message);
    //             self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    //             break;
    //         }
    //
    //         if let Ok(message_length) = self.buf_reader.read_line(&mut received_message) {
    //             if message_length > 1 {
    //                 let all_messages: Vec<&str> = received_message.trim().split("#").collect();
    //                 for message in all_messages {
    //                     println!("Message {:?}", message);
    //                     match message {
    //                         AppDefines::QUIT => {
    //                             let message = StyledMessage {
    //                                 text: format!("[INFO] Client exited the game: {}", self.socket.peer_addr().unwrap()),
    //                                 color: egui::Color32::from_rgb(0, 0, 255), // Blue color for exit messages
    //                             };
    //                             self.messages.lock().unwrap().push(message);
    //
    //                             self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    //                             running = false;
    //                             break;
    //                         }
    //                         _ => {
    //                             println!("Default TODO PROCESS MESSAGE");
    //                             self.previous_time = current_time;
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 received_message.clear();
    //             } else {
    //                 println!("Client disconnected: {:?}", self.socket.peer_addr().unwrap());
    //                 let message = StyledMessage {
    //                     text: format!("[INFO] Client disconnected: {}", self.socket.peer_addr().unwrap()),
    //                     color: egui::Color32::from_rgb(255, 165, 0), // Orange color for disconnection messages
    //                 };
    //                 self.messages.lock().unwrap().push(message);
    //
    //                 self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    //                 running = false;
    //                 break;
    //             }
    //         }
    //     }
    // }

    pub fn make_name_list(){
    }

    pub fn add_to_reponse(mut reponse: String, message: String) {
        if (reponse != "") {
            reponse += "#";
            return reponse += &*message;
        }
    }

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