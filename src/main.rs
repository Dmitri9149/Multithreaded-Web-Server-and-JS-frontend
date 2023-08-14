use std::{
    fs,    
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const HTTP: &str = "HTTP/1.1 200 OK";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn get_path(path:&str) -> String {
    return format!("GET {path} HTTP/1.1");
}

fn file_path(folder:&str, file:&str) -> String {
    return format!("{folder}{file}"); 
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = 
    if request_line == get_path("/") { //"GET / HTTP/1.1" {
        (HTTP, file_path("paths/", "hello.html"))
    } else if request_line == get_path("/news") {
        (HTTP, file_path("paths/", "news.html"))
    } else if request_line == get_path("/resources") {
        (HTTP, file_path("paths/", "resources.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", file_path("paths/", "404.html"))
    };

    let contents = fs::read_to_string(&filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("Request_line {:?}", &request_line);
    println!("Stream {:?}", &stream);
    println!("Lines {} and {}", &status_line, &filename);
    println!("Response {}", response);
    stream.write_all(response.as_bytes()).unwrap();
}