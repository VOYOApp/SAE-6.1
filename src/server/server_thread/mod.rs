use std::net::{TcpListener};
use std::thread;
use std::time::Duration;
use crate::server::client_handler;

use crate::server::client_handler::{ClientHandler};

pub(crate) struct ServerThread {
    pub(crate) port: u16,
}

impl ServerThread {
    pub(crate) fn new(port: u16) -> Self {
        ServerThread { port }
    }

    pub(crate) fn start(&self) {
        let listener = TcpListener::bind(("127.0.0.1", self.port)).expect("Could not bind to port");

        println!("Server address: {:?}", listener.local_addr().unwrap());
        println!("Listening on port: {} \n", self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected: {}", stream.peer_addr().unwrap());
                    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap(); // Set timeout
                    thread::spawn(move || {
                        ClientHandler::new(stream).run();
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}
