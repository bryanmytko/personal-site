use std::{
    fs,
    io::{prelude::*, BufReader },
    net::TcpStream,
};
use rand::Rng;
use regex::Regex;

pub fn route(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_split: Vec<&str> = request_line.split_whitespace().collect();

    let verb = request_split[0];
    let path = request_split[1];

    match verb {
        "GET" => get(stream, path),
        _ => not_found(stream)
    }
}

fn get(mut stream: TcpStream, path: &str) {
    let asset_path: &str = "assets";
    let image_regex = Regex::new(r".*\.(png|gif|jpg|jpeg).*").unwrap();
    let path_split: Vec<&str> = path.split("/").collect();

    /* Need to implement some kind of actual routes to check here */
    if path == "/" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("html/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else if path_split[1] == asset_path && image_regex.is_match(path) {
        asset(stream, path);
    } else {
        not_found(stream);
    }
}

fn not_found(mut stream: TcpStream) {
    let page = fs::read_to_string("html/404.html").unwrap();
    let status_line = "HTTP/1.1 400 NOT FOUND";
    let length = page.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{page}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn asset(mut stream: TcpStream, _path: &str) {
    let logo = random_logo();
    let file_path = format!("assets/{}", logo);
    let contents = fs::read(file_path).unwrap();
    let status_line = "HTTP/1.1 200 OK";

    // @TODO
    // let headers = [
    //     "HTTP/1.1 200 OK",
    //     "Content-type: image/jpeg",
    //     "Transfer-Encoding: chunked",
    //     "\r\n"
    // ];

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        contents.len()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
}

/* Move to front-end once this is supported */
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
