use std::{
    fs,
    fs::{File},
    io::{prelude::*, BufReader },
    net::{TcpListener, TcpStream},
};

use chunked_transfer::Encoder;

static LISTEN_ADDR: &str = "0.0.0.0:3000";

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR).unwrap();

    for stream in listener.incoming() {
        let stream  = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("html/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let file_path = format!("assets/wordart.png");
        let mut buf = Vec::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_end(&mut buf).unwrap();
        let mut encoded = Vec::new();

        {
            let mut encoder = Encoder::with_chunks_size(&mut encoded, 8);
            encoder.write_all(&buf).unwrap();
        }

        let headers = [
            "HTTP/1.1 200 OK",
            "Content-type: image/jpeg",
            "Transfer-Encoding: chunked",
            "\r\n"
        ];

        let mut response = headers.join("\r\n")
            .to_string()
            .into_bytes();

        response.extend(encoded);

        stream.write_all(&response).unwrap();
        match stream.write(&response) {
            Ok(_) => println!("OK!"),
            Err(_) => println!("Error: something went wrong!"),
        }
    }

    // println!(">>> {}", request_line);
}
