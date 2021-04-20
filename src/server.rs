use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;

extern crate env_logger;
extern crate httparse;
extern crate threadpool;

use httparse::Request;
use log::{error, info, warn};
use threadpool::ThreadPool;

use crate::cli;
use crate::utils;

struct Config {
    dir: PathBuf,
    index: String,
    not_found: Option<String>,
    uri: String,
}

impl Config {
    fn new(args: cli::Args) -> Config {
        Config {
            dir: args.dir,
            index: args.index,
            not_found: args.not_found,
            uri: format!("{}:{}", args.addr, args.port),
        }
    }
}

enum RequestState {
    NotProcessed,
    ParseError,
    BadRequest,
    FileNotFound,
    FileFound(PathBuf),
}

pub fn serve(args: cli::Args) {
    let config = Arc::new(Config::new(args));

    let listener = TcpListener::bind(&config.uri).unwrap();
    let pool = ThreadPool::new(10);

    info!(
        "Serving files at {} with {} workers",
        config.uri,
        pool.max_count()
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let cloned_config = Arc::clone(&config);

        pool.execute(move || handle_connection(stream, cloned_config));
    }

    pool.join();
}

fn handle_connection(mut stream: TcpStream, config: Arc<Config>) {
    // Store state/stage of request processing
    let mut state = RequestState::NotProcessed;

    // Reading stream
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    // Parse request using httparse
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    if let Err(e) = req.parse(&buf) {
        error!("Error parsing request: {}", e);
        state = RequestState::ParseError;
    }

    // Incomplete Request
    if let None = req.path {
        warn!("Bad Request");
        state = RequestState::BadRequest;
    }

    // Preprocess path and obtain file path
    let url_path = req.path.unwrap().replace("../", "").replace("%20", " ");
    let file_path: PathBuf = match url_path.as_str() {
        "/" => config.dir.join(&config.index),
        s => config.dir.join(&s[1..]),
    };

    info!("Requesting {:?}", file_path);

    // Check if path exists
    if file_path.exists() {
        state = RequestState::FileFound(file_path);
    } else if !config.not_found.is_none() {
        state = RequestState::FileFound(config.dir.join(config.not_found.as_ref().unwrap()));
    } else {
        warn!("File not found");
        state = RequestState::FileNotFound;
    }

    let response = match state {
        RequestState::ParseError | RequestState::NotProcessed => {
            b"HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n".to_vec()
        }
        RequestState::BadRequest => b"HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_vec(),
        RequestState::FileNotFound => b"HTTP/1.1 404 NOT FOUND\r\n\r\n".to_vec(),
        RequestState::FileFound(path) => {
            let mut contents = fs::read(&path).unwrap();
            let contents_len = contents.len();
            let contents_type =
                utils::extension_to_mime(path.extension().and_then(std::ffi::OsStr::to_str));
            let mut bytes: Vec<u8> = format!(
                "HTTP/1.1 200 OK\r\nContent-type: {}\r\nContent-Length: {}\r\n\r\n",
                contents_type, contents_len
            )
            .as_bytes()
            .to_vec();
            bytes.append(&mut contents);

            bytes
        }
    };

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}
