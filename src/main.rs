use std::{
    fs,
    io::{prelude::*, BufReader },
    net::{TcpListener, TcpStream},
};

use rand::Rng;
use regex::Regex;

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
    let image_regex = Regex::new(r"GET.*\.(png|gif|jpg|jpeg).*").unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("html/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else if image_regex.is_match(&request_line) {
        let logo = random_logo();
        let file_path = format!("assets/{}", logo);
        let contents = fs::read(file_path).unwrap();

        let status_line = "HTTP/1.1 200 OK";

        let response = format!("{}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            contents.len()
        );

        // @TODO
        // let headers = [
        //     "HTTP/1.1 200 OK",
        //     "Content-type: image/jpeg",
        //     "Transfer-Encoding: chunked",
        //     "\r\n"
        // ];
        
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
    }
}

fn random_logo() -> &'static str {
    let logos = [
        "wordart1.png",
        "wordart2.png",
        "wordart3.png",
        "wordart4.png",
        "wordart5.png",
        "wordart.gif",
    ];

    let mut rng = rand::thread_rng();
    let rng_index = rng.gen_range(1..logos.len());

    return logos[rng_index];
}
