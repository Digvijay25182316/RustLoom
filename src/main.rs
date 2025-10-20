use std::error::Error;
use std::net::TcpListener;
use std::thread;
mod controllers;
mod network_handler;
mod router;
mod services;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("🚀 Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    // <-- important move
                    if let Err(e) = network_handler::network_handler(stream) {
                        eprintln!("⚠️ Connection error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
