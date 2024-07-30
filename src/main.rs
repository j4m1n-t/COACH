use std::net::TcpStream;
use std::io::{self,Error};
use std::env;

fn establish_connection(host: &str) -> Result<TcpStream, io::Error>{
    // Establish a connection to the specified host using a secure connection protocol (e.g., TLS/SSL)
    // Return a connection object or an error message if the connection fails
   let connection = TcpStream::connect(host);
    match connection {
        Ok(conn) => Ok(conn),
        Err(err) => Err(err),
    }
}


fn main() {
    // Later we need to implement -t to allow targets for a file of targets

    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} -h <host> -p <port>", args[0]);
        return;
    }

    let mut host = "";
    let mut port = "";

    for i in 0..args.len() {
        match args[i].as_str() {
            "-h" => host = &args[i + 1],
            "-p" => port = &args[i + 1],
            _ => {}
        }
    }

    let full_host = format!("{}:{}", host, port);

    match establish_connection(&full_host) {
        Ok(stream) => println!("Successfully connected to {}", full_host), 
        Err(e) => println!("Failed to connect to {}", e),

    }
}
