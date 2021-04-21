//! Command line arguments, parsing, and verification

use argparse::{ArgumentParser, Store, StoreOption};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Clone)]
/// Struct for cli arguments.
pub struct Args {
    /// Path of directory to be served.
    pub dir: PathBuf,
    /// Index file name in `dir`.
    pub index: String,
    /// 404 file name in `dir`. If `None`, the 404 error message is returned
    pub not_found: Option<String>,
    /// Address for serving
    pub address: SocketAddr,
    /// IP for serving
    pub ip: String,
    /// Port for serving
    pub port: u16,
}

impl Args {
    /// Generate `Args` instance with default values:
    /// - `dir`: current directory (`./`)
    /// - `index`: `index.html` in the current directory
    /// - `not_found`: `None` (404 error is returned)
    /// - `address`: 127.0.0.1:7878
    /// - `ip`: 127.0.0.1
    /// - `port`: 7878
    pub fn new() -> Args {
        Args {
            dir: PathBuf::from("./"),
            index: String::from("index.html"),
            not_found: None,
            address: "127.0.0.1:7878".parse().unwrap(),
            ip: String::from("127.0.0.1"),
            port: 7878,
        }
    }

    /// Parsing involving storing arguments and verifying them.
    /// Returns `Err` variant depending on return value of `Args.verify`
    pub fn parse(&mut self) -> Result<(), &str> {
        self.store();
        self.verify()
    }

    /// Verify if the arguments given are valid.
    /// Checks if dir, index, and 404 files (if given) exist
    /// and if the address (port and ip) are valid.
    pub fn verify(&mut self) -> Result<(), &'static str> {
        // Dir check
        if !self.dir.is_dir() {
            return Err("Directory does not exist");
        };
        // Index check
        if !self.dir.join(&self.index).is_file() {
            return Err("Index file does not exist in given directory");
        };
        // 404 check
        if !self.not_found.is_none() && !self.dir.join(self.not_found.as_ref().unwrap()).is_file() {
            return Err("404 file does not exist in given directory");
        };
        // Address check
        let address: String = format!("{}:{}", self.ip, self.port);
        if address.parse::<SocketAddr>().is_err() {
            return Err("Invalid IP Address given");
        } else {
            self.address = address.parse::<SocketAddr>().unwrap();
        }

        Ok(())
    }

    /// Store arguments using the `argparse` crate.
    pub fn store(&mut self) {
        let mut parser = ArgumentParser::new();
        parser.set_description("Minimal server for yew and yew-router apps");

        parser.refer(&mut self.dir).add_option(
            &["-d", "--dir"],
            Store,
            "Directory to serve (must contain index.html). Default: current directory",
        );
        parser.refer(&mut self.index).add_option(
            &["-i", "--index"],
            Store,
            "Index file to be served. Default: index.html",
        );
        parser.refer(&mut self.not_found).add_option(
            &["-n", "--404", "--not-found"],
            StoreOption,
            "File to be served in case of 404",
        );
        parser.refer(&mut self.ip).add_option(
            &["-a", "--addr"],
            Store,
            "Address at which files are served. Default: localhost",
        );
        parser.refer(&mut self.port).add_option(
            &["-p", "--port"],
            Store,
            "Port at which files are served. Default: 7878",
        );

        parser.parse_args_or_exit();
    }
}
