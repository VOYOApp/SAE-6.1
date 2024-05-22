use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::ptr::null;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::app_defines::AppDefines;

const CONNECTION_COUNT: u64 = 0;
const INTERRUPT: bool = false;
// const bot: BotBody2D;

pub(crate) struct ClientHandler {
    pub(crate) socket : TcpStream,
    pub(crate) buf_writer: BufWriter<TcpStream>,
    pub(crate) buf_reader: BufReader<TcpStream>,
    pub(crate) previous_time: u64,
}

impl ClientHandler {
    pub fn new(socket: TcpStream) -> Self {
        let buf_writer = BufWriter::new(socket.try_clone().unwrap());
        let buf_reader = BufReader::new(socket.try_clone().unwrap());
        ClientHandler {
            socket,
            buf_writer,
            buf_reader,
            previous_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    // println!("{:?} {:?} {:?}", now.duration_since(UNIX_EPOCH).unwrap().as_secs(), previous_time, AppDefines::CONNECTION_TIMEOUT_DELAY as u64);

    pub fn run(&mut self) {
        let mut received_message = String::new();
        let mut running = true;

        while running {
            let now = SystemTime::now();
            let current_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

            // println!("{:?} {:?} {:?}", current_time, self.previous_time, AppDefines::CONNECTION_TIMEOUT_DELAY as u64);
            // println!("{:?}", current_time - self.previous_time);

            if current_time - self.previous_time > AppDefines::CONNECTION_TIMEOUT_DELAY as u64 {
                println!("Connection timeout {:?}", self.socket.peer_addr().unwrap());
                self.socket.shutdown(std::net::Shutdown::Both).unwrap();
                break;
            }

            if let Ok(message_length) = self.buf_reader.read_line(&mut received_message) {
                if message_length > 1 {
                    let all_messages: Vec<&str> = received_message.trim().split("#").collect();
                    for message in all_messages {
                        println!("Message {:?}", message);
                        match message {
                            AppDefines::QUIT => {
                                println!("LEAVE THE GAME");
                                self.socket.shutdown(std::net::Shutdown::Both).unwrap();
                                running = false;
                                break;
                            }
                            _ => {
                                println!("Default TODO PROCESS MESSAGE");
                                self.previous_time = current_time;
                                break;
                            }
                        }
                    }
                    received_message.clear();
                } else {
                    println!("Client disconnected: {:?}", self.socket.peer_addr().unwrap());
                    self.socket.shutdown(std::net::Shutdown::Both).unwrap();
                    running = false;
                    break;
                }
            }
        }
    }


    // pub fn close_socket(&self) {
    //     println!("Closing this connection");
    //     if (self.socket != null()) {
    //         self.socket.shutdown(std::net::Shutdown::Both).unwrap();
    //     }
    //     println!("Connection closed");
    // }

    pub fn make_name_list(){
    }

    pub fn add_to_reponse(mut reponse: String, message: String) {
        if (reponse != "") {
            reponse += "#";
            return reponse += &*message;
        }
    }

    pub fn leave_game() -> bool {
        // CONNECTION_COUNT -= 1;
        // TODO ESSAYE D'ENLEVER UN BOT SUR LA MAP
        return true;
    }

    pub fn join_game() -> bool {
        // CONNECTION_COUNT += 1;
        // TODO ESSAYE D'AJOUTET UN BOT SUR LA MAP
        return true;
    }

    // pub fn setBot(bot){
    // }

    pub fn make_default_name() -> String {
        return "Player ".to_string() + &*CONNECTION_COUNT.to_string();
    }

    // pub fn is_offline() -> bool {
    // }
}

pub fn close_socket(client : ClientHandler) {
    println!("Closing this connection");
    client.socket.shutdown(std::net::Shutdown::Both).unwrap();
    println!("Connection closed");
}


// pub(crate) fn client_handler(stream: TcpStream) {
//     let peer_addr = match stream.peer_addr() {
//         Ok(addr) => addr,
//         Err(e) => {
//             println!("Could not get peer address: {}", e);
//             return;
//         }
//     };
//
//     println!("Handling client: {} \n", peer_addr);
//
//     let reader = BufReader::new(&stream);
//     let mut writer = BufWriter::new(&stream);
//
//     for line in reader.lines() {
//         match line {
//             Ok(line) => {
//                 println!("Received from {}: {}", peer_addr, line.trim());
//                 if line.trim().is_empty() {
//                     println!("Empty message received, closing connection: {}", peer_addr);
//                     break;
//                 }
//                 if let Err(e) = writer.write_all(line.as_bytes()) {
//                     println!("Failed to write to stream: {}", e);
//                     break;
//                 }
//                 if let Err(e) = writer.write_all(b"\n") {
//                     println!("Failed to write newline: {}", e);
//                     break;
//                 }
//                 if let Err(e) = writer.flush() {
//                     println!("Failed to flush stream: {}", e);
//                     break;
//                 }
//             }
//             Err(e) => {
//                 if e.kind() == std::io::ErrorKind::WouldBlock {
//                     continue;
//                 }
//                 println!("Error reading from {}: {}", peer_addr, e);
//                 break;
//             }
//         }
//     }
//
//     println!("Client disconnected: {}", peer_addr);
// }