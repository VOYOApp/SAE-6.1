use std::net::{TcpListener};

pub(crate) struct ServerThread {
    pub(crate) port: u16,
}

impl ServerThread {
    pub(crate) fn new(port: u16){
        ServerThread{
            port
        };
    }
    pub(crate) fn start(&self) {
        let listener = TcpListener::bind(("0.0.0.0", self.port));
        println!("Server address: {:?}", listener.expect("Connection failed").local_addr());
        println!("Listening on port: {}", self.port);
    }
}