use std::{ env, net::TcpListener };
use dotenv;

mod http;

use personal_website::ThreadPool;

fn main() {
    dotenv::dotenv().ok();

    let port: String = match env::var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "5000".to_string()
    };
    let host: String = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&host).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream  = stream.unwrap();

        pool.execute(|| { http::handle_request(stream) });
    }

    println!("Shutting down...");
}
