use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;

extern crate httparse;
extern crate threadpool;

use httparse::Request;
use threadpool::ThreadPool;

use crate::cli;

struct Config {
    dir: PathBuf,
    index: String,
}

impl Config {
    fn new(args: cli::Args) -> Config {
        Config {
            dir: args.dir,
            index: args.index,
        }
    }
}

pub fn serve(args: cli::Args) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    println!(
        "Serving files at {} with {} workers",
        "127.0.0.1:7878",
        pool.max_count()
    );

    let state = Arc::new(Config::new(args));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let cloned_state = Arc::clone(&state);

        pool.execute(move || handle_connection(stream, cloned_state));
    }

    pool.join();
}

fn handle_connection(mut stream: TcpStream, config: Arc<Config>) {
    // Reading stream
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    // Parse request using httparse
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    if let Err(e) = req.parse(&buf) {
        println!("Error parsing request: {}", e);
        return;
    }

    // Read file if exists
    let response = match req.path {
        Some("/") => {
            format!(
                "HTTP/1.1 200 OK\r\n\r\n{}",
                fs::read_to_string(config.dir.join(&config.index)).unwrap()
            )
        }
        Some(s) => {
            let path = config.dir.join(&s[1..]);
            println!("Requesting {:?}", path);
            if path.exists() {
                format!(
                    "HTTP/1.1 200 OK\r\n\r\n{}",
                    fs::read_to_string(path).unwrap()
                )
            } else {
                String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
            }
        }
        None => {
            println!("Bad Request.",);
            String::from("HTTP/1.1 400 BAD REQUEST\r\n\r\n")
        }
    };

    println!("{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
