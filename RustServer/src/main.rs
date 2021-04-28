#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7070").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(s) => {
        handle_connection(s);
      }
      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        println!("Exit");
        break;
      }
      Err(e) => panic!("encountered IO error: {}", e),
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let contents = "<!DOCTYPE html>
    <html lang='en'>
      <head>
        <meta charset='utf-8'>
        <title>Hello!</title>
      </head>
      <body>
        <h1>Hello!</h1>
        <p>Hi from Rust</p>
      </body>
    </html>";

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
