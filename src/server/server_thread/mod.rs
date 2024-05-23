use std::net::{TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::egui;

use crate::server::client_handler::{ClientHandler};
use crate::StyledMessage;

pub(crate) struct ServerThread {
    pub(crate) port: u16,
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
}

impl ServerThread {
    pub(crate) fn new(port: u16, messages: Arc<Mutex<Vec<StyledMessage>>>)-> Self {
        ServerThread { port, messages }
    }

    pub(crate) fn start(&self) {
        let listener = TcpListener::bind(("127.0.0.1", self.port)).expect("Could not bind to port");

        let message = StyledMessage {
            text: format!("[START] Server address: {:?}", listener.local_addr().unwrap()),
            color: egui::Color32::from_rgb(0, 255, 0),
        };
        self.messages.lock().unwrap().push(message);
        let message2 = StyledMessage {
            text: format!("[START] Listening on port: {}", self.port),
            color: egui::Color32::from_rgb(0, 255, 0),
        };
        self.messages.lock().unwrap().push(message2);
        println!("Server address: {:?}", listener.local_addr().unwrap());
        println!("Listening on port: {} \n", self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let messages = Arc::clone(&self.messages);
                    let message = StyledMessage {
                        text: format!("[INFO] New client connected: {}", stream.peer_addr().unwrap()),
                        color: egui::Color32::from_rgb(0, 255, 0), // Green color for start messages
                    };
                    messages.lock().unwrap().push(message);
                    println!("New client connected: {}", stream.peer_addr().unwrap());
                    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap(); // Set timeout
                    thread::spawn(move || {
                        ClientHandler::new(stream, messages).run();
                    });
                }
                Err(e) => {
                    let messages = Arc::clone(&self.messages);
                    let message = StyledMessage {
                        text: format!("[ERROR] Connection failed: {}", e),
                        color: egui::Color32::from_rgb(255, 0, 0), // Red color for error messages
                    };
                    messages.lock().unwrap().push(message);
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}
