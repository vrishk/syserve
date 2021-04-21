//! Serving files from a directory

use std::fs;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
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

/// Basic configuration for server. Derived from `cli::Args`
pub struct Config {
    /// Path of directory to be served.
    dir: PathBuf,
    /// Index file name in `dir`.
    index: String,
    /// 404 file name in `dir`. If `None`, the 404 error message is returned
    not_found: Option<String>,
    /// Address for serving
    address: SocketAddr,
}

impl Config {
    pub fn new(args: cli::Args) -> Config {
        Config {
            dir: args.dir,
            index: args.index,
            not_found: args.not_found,
            address: args.address,
        }
    }
}

/// State of a request while being processed
pub enum RequestState {
    NotProcessed,
    ParseError,
    BadRequest,
    FileNotFound,
    FileFound(PathBuf),
}

/// Main serve function for the binary from `cli::Args`
pub fn serve(args: cli::Args) {
    let config = Arc::new(Config::new(args));

    let listener = TcpListener::bind(&config.address).unwrap();
    let pool = ThreadPool::new(10);

    info!(
        "Serving files at {} with {} workers",
        config.address,
        pool.max_count()
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let cloned_config = Arc::clone(&config);

        pool.execute(move || handle_connection(stream, cloned_config));
    }

    pool.join();
}

/// Handle connection/request. Spawned as part of a thread.
/// Returns 4 possible responses:
/// - Internal Server Error (500) if request parsing fails
/// - Bad Request (400) if the request has no path
/// - Not Found (404) if the requested path is not found and if `Config::not_found` is `None`
/// or if the `Config::not_found` file does not exist in `Config::dir`
/// - Ok (200) if the requested path is found or if the `Config::not_found` file exists in `Config::dir`

pub fn handle_connection(mut stream: TcpStream, config: Arc<Config>) {
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
