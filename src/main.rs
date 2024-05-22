mod server;
use crate::server::server_thread::ServerThread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serv = ServerThread::new(6969);
    serv.start();
    Ok(())
}

