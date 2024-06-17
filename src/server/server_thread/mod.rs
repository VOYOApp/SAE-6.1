use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::server::client_handler::ClientHandler;
use crate::types::{add_message, MessageType, StyledMessage};

pub(crate) struct ServerThread {
    pub(crate) port: u16,
    pub(crate) messages: Arc<Mutex<Vec<StyledMessage>>>,
}

impl ServerThread {
    pub(crate) fn new(port: u16, messages: Arc<Mutex<Vec<StyledMessage>>>)-> Self {
        ServerThread { port, messages }
    }

    pub(crate) async fn start(&self) {
        let listener = TcpListener::bind(("127.0.0.1", self.port)).expect("Could not bind to port");

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
                    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap(); // Set timeout
                    thread::spawn(move || {
                        ClientHandler::new(stream, messages).run();
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
