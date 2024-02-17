use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
fn main() {
    let listenerRes = TcpListener::bind("127.0.0.1:8000");
    match listenerRes {
        Ok(listener) => startAcceptingConn(listener),
        Err(err) => println!("Bad luck buddy: {}", err.to_string()),
    };
    println!("Hello, world!");
}

fn startAcceptingConn(listener: TcpListener) {
    println!("Server has started");
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("Incoming connection addr {}", addr.to_string());
            let mut buffer: [u8; 128] = [0; 128];
            let mut mstream = stream;
            loop {
                match mstream.read(&mut buffer) {
                    Ok(readBytes) => {
                        println!("Read {} bytes so far", readBytes);
                        match mstream.write(&mut buffer[0..readBytes]) {
                            Ok(writeBytes) => println!("{} bytes were written", writeBytes),
                            Err(err) => println!("Not Again: {}", err.to_string()),
                        }
                    }
                    Err(err) => println!("Something went wrong: {}", err.to_string()),
                }
            }
        }
        Err(err) => println!("Badluck Again {}", err.to_string()),
    }
}
