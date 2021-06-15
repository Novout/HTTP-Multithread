use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::env;
use std::str;
use chrono::{Utc};
use sysinfo::{System, SystemExt};
use http_multithread::ThreadPool;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let ref port: &String = &args[1];

    let listener = TcpListener::bind("127.0.0.1:".to_owned() + port).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

#[allow(unused_mut, unused_variables)]
fn handle_connection(mut stream: TcpStream) -> () {
    let mut buffer = [0; 1024];
    let mut sys = System::new_all();

    stream.read(&mut buffer).unwrap();

    let request = str::from_utf8(&buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/views/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/views/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let server_response = format!(
        "Server: {}{} ({})",
        "Apache ",
        sys.get_os_version().unwrap(),
        sys.get_name().unwrap()
    );

    let date_response = format!(
        "Date: {}",
        Utc::now().to_rfc2822()
    );

    let response = format!(
        "{}\r\nConnection: keep-alive\r\n{}\r\n{}\r\nAccept-Language: *\r\nAccess-Control-Allow-Origin: *\r\nContent-Lenght: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
        status_line,
        date_response,
        server_response,
        contents.len(),
        contents
    );

    /*
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    */

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}