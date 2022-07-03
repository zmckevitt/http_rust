use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::str;
use std::fs;
//use std::fs::File;

const FILE:&str = "html/index.html";
const BUFSIZE: usize = 512;

// TODO: handle different page reqs
// currently just serving index.html by default
fn handle(mut stream: TcpStream) -> io::Result<()> {


    // declare buffer for writing html file to client
    let mut buffer = [0 as u8; BUFSIZE];
    
    // open file
    let mut myfile = fs::File::open(FILE)?;

    // get size of file
    let metadata = fs::metadata(FILE);
    let filesize = metadata.unwrap().len();

    // connstruct http header
    let http_header = format!("HTTP/1.1 200 OK\r\nContent-Type:text/html\r\n\
                      Content-Length:{}\r\nConnection:keep-alive\r\n\r\n", filesize);
    println!("{}", http_header);
    stream.write(&http_header.as_bytes()).unwrap();            
    
    loop {
        let read_count = myfile.read(&mut buffer)?;
        stream.write(&buffer[..read_count]).unwrap();
        if read_count != BUFSIZE {
            break;
        }
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    // dispatch connections to threads
    println!("Server listening on port 8080...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // successful connection
                    handle(stream)
                });
            }
            Err(e) => {
                // conn failed
                println!("Error: {}", e);
            }
        }
    }
    // close socket
    drop(listener);
}
