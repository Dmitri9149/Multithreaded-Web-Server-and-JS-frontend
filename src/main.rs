use std::{
    fs,    
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = 
    if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "paths/hello.html")
    } else if request_line == "GET /news HTTP/1.1" {
        ("HTTP/1.1 200 OK", "paths/news.html")
    } else if request_line == "GET /resources HTTP/1.1" {
        ("HTTP/1.1 200 OK", "paths/resources.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "paths/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Request_line {:?}", &request_line);
    println!("Stream {:?}", &stream);
    println!("Lines {} and {}", &status_line, &filename);
    println!("Response {}", response);
    stream.write_all(response.as_bytes()).unwrap();
}