use std::net::{TcpListener, TcpStream};
use std::io::{Result, Write, Read};
use std::{str, thread};
use std::net::Shutdown;

fn handle_client(mut stream: TcpStream) {
    let mut request_string = String::new();
    let mut buf = [0; 10];
    loop {
        match stream.read(&mut buf) {
            Ok(amount) => {
                if amount == 0 { break; }
                match String::from_utf8(buf.to_vec()) {
                    Ok(s) => request_string += &s,
                    Err(e) => {
                        println!("Error: {}", e);
                        println!("Buffer: {:?}", buf);
                        break;
                    }
                }
                if request_string.contains("\r\n\r\n"){ break; }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        };
    };
    let response: &str = "HTTP/1.1 200 OK\r\nContent-Length: 1\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\na";
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Send response"),
        Err(e) => println!("Error: {}", e)
    }
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => println!("Shutdown succeeded."),
        Err(_) => println!("Shutdown failed.")
    };
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    for stream in listener.incoming() {
        match stream {
            Ok(ok_stream) => { thread::spawn(move || { handle_client(ok_stream); });},
            Err(e) => println!("Error: {}", e)
        }
    }
    Ok(())
}
