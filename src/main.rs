use std::{ env, net::TcpListener };
use dotenv;

mod http;

fn main() {
    dotenv::dotenv().ok();

    let port: String = match env::var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "5000".to_string()
    };
    let host: String = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&host).unwrap();

    for stream in listener.incoming() {
        let stream  = stream.unwrap();

        http::handle_request(stream);
    }
}
