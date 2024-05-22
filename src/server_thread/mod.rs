use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::thread;
use std::time::Duration;

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
                    stream.set_read_timeout(Some(Duration::from_millis(5000))).unwrap(); // Set timeout
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Could not get peer address: {}", e);
            return;
        }
    };

    println!("Handling client: {} \n", peer_addr);

    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("Received from {}: {}", peer_addr, line.trim());
                if line.trim().is_empty() {
                    println!("Empty message received, closing connection: {}", peer_addr);
                    break;
                }
                if let Err(e) = writer.write_all(line.as_bytes()) {
                    println!("Failed to write to stream: {}", e);
                    break;
                }
                if let Err(e) = writer.write_all(b"\n") {
                    println!("Failed to write newline: {}", e);
                    break;
                }
                if let Err(e) = writer.flush() {
                    println!("Failed to flush stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    continue;
                }
                println!("Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }

    println!("Client disconnected: {}", peer_addr);
}
