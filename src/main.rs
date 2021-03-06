/*
 * HTTP Server in Rust
 * Zack McKevitt, 2022
 *
 *
 * TODO:
 * Parse requests into struct of components
 * Error handling
 *
*/

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::str;
use std::fs;

const BUFSIZE: usize = 512;


/*
 * Request struct
 *
 * Parse request string into
 * Request type
 * Requested resource
 * HTTP version
 * Host address
 * Connection type 
 *
 */
struct Request {
    req_type:  String,
    resource:  String,
    version:   String,
    host_addr: String,
    conn_type: String,
}

/*
 * parseRequest
 *
 * Parse request string, populate Request struct
 *
 */
fn parse_request(http_request: &str) -> Request{

    let tokens: Vec<&str> = http_request.split("\r\n").collect(); 
    let first_line: Vec<&str> = tokens[0].split(" ").collect();
    let second_line: Vec<&str> = tokens[1].split(" ").collect();
    let final_line: Vec<&str> = tokens[2].split(" ").collect();
    
    let req = Request {
        req_type:  first_line[0].to_string(),
        resource:  first_line[1].to_string(),
        version:   first_line[2].to_string(),
        host_addr: second_line[1].to_string(),
        conn_type: final_line[1].to_string(),
    };
    
    req
}

fn get_conn_type(filename: &str) -> &str {
    
    let extension: Vec<&str> = filename.split(".").collect();
    println!("{}", extension[1]);
    match extension[1] {
        
        "html" => { "text/html" }
        "css" =>  { "text/css" }
        
        "ico" => { "image/x-icon" }
        "gif" => { "image/gif" }
        "jpg" => { "image/jpeg"}
        "png" => { "image/png" }

        "js" =>  { "application/javascript" }

        _ =>     { "text/html" }
    }
}

/*
 * handle
 *
 * Thread function to handle incoming requests
 * Parse each result and transmit desired resource
 *
 */
fn handle(mut stream: TcpStream) -> io::Result<()> {

    // parse the request
    let mut request_dat = [0 as u8; BUFSIZE];
    stream.read(&mut request_dat)?;
    let request_str = str::from_utf8(&request_dat).unwrap();
    let http_request: Request = parse_request(request_str);

    println!("Original Request:\n{}", request_str);

    // declare buffer for writing html file to client
    let mut buffer = [0 as u8; BUFSIZE];
   
    // open file
    let mut myfile;
    let fname: String;
    let cont_type: String;
    if http_request.resource.eq("/") {
        fname = "html/index.html".to_owned();
        cont_type = "text/html".to_owned();
        myfile = fs::File::open(&fname)?;
    } else {
        let mut path_to_res: String = "html/".to_owned();
        path_to_res.push_str(&http_request.resource.to_owned());
        fname = path_to_res;
        cont_type = get_conn_type(&fname.to_string()).to_string();
        myfile = fs::File::open(&fname.to_owned())?; 
    }

    // get size of file
    let metadata = fs::metadata(fname);
    let filesize = metadata.unwrap().len();

    // connstruct http header
    let http_header = format!("HTTP/1.1 200 OK\r\nContent-Type:{}\r\nContent-Length:{}\r\n\
                              Connection:{}\r\n\r\n", cont_type, filesize, http_request.conn_type);
    
    println!("{}-------------------------------", http_header);
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
