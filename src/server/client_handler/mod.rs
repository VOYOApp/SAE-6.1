use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};
pub(crate) fn client_handler(stream: TcpStream) {
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
