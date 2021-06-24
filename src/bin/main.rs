use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::env;
use std::str;
use std::panic;
use chrono::{Utc};
use sysinfo::{System, SystemExt};
use http_multithread::ThreadPool;
extern crate image_base64;

static DEFAULT_GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";
// static DEFAULT_POST: &[u8; 17] = b"POST / HTTP/1.1\r\n";
// static DEFAULT_PUT: &[u8; 16] = b"PUT / HTTP/1.1\r\n";
// static DEFAULT_DELETE: &[u8; 19] = b"DELETE / HTTP/1.1\r\n";
static CONTENT_TYPE_HTML: &'static str = "Accept: text/html";
static CONTENT_TYPE_PLAIN: &'static str = "Accept: text/plain";
static CONTENT_TYPE_CSS: &'static str = "Accept: text/css";
static CONTENT_TYPE_JS: &'static str = "Accept: text/javascript";
static CONTENT_TYPE_MD: &'static str = "Accept: text/markdown";
static CONTENT_TYPE_JSON: &'static str = "Accept: application/json";
static CONTENT_TYPE_JPEG: &'static str = "Accept: image/jpeg";

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
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut sys: sysinfo::System = System::new_all();

    stream.read(&mut buffer).unwrap();

    let request: &str = str::from_utf8(&buffer).unwrap();

    println!("{}", request);

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

    let (status_line, filename) = if buffer.starts_with(DEFAULT_GET) {
        ("HTTP/1.1 200 OK", "src/views/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/views/404.html")
    };

    let (contents, content_type) = if request.contains(CONTENT_TYPE_HTML) {
        (fs::read_to_string(filename).unwrap(), "text/html")
    } else if request.contains(CONTENT_TYPE_PLAIN) {
        (fs::read_to_string(filename).unwrap(), "text/plain")
    }  else if request.contains(CONTENT_TYPE_CSS) {
        (fs::read_to_string("src/data/style.css").unwrap(), "text/css")
    } else if request.contains(CONTENT_TYPE_JS) {
        (fs::read_to_string("src/data/index.js").unwrap(), "text/javascript")
    } else if request.contains(CONTENT_TYPE_MD) {
        (fs::read_to_string("src/data/README.md").unwrap(), "text/markdown")
    } else if request.contains(CONTENT_TYPE_JSON) {
        (fs::read_to_string("src/data/base.json").unwrap(), "application/json")
    } else if request.contains(CONTENT_TYPE_JPEG) {
        (image_base64::to_base64("src/assets/ifmg.jpg"), "image/jpeg")
    } else {
        panic!();
    };

    let response = format!(
        "{}\r\nConnection: keep-alive\r\n{}\r\n{}\r\nAccept-Language: *\r\nAccess-Control-Allow-Origin: *\r\nContent-Lenght: {}\r\nContent-Type: {}; charset=utf-8\r\n\r\n{}",
        status_line,
        date_response,
        server_response,
        contents.len(),
        content_type,
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