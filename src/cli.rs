use argparse::{ArgumentParser, Store, StoreOption};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Args {
    pub dir: PathBuf,
    pub index: String,
    pub not_found: Option<String>,
    pub port: u32,
    pub addr: String,
}

impl Args {
    pub fn new() -> Args {
        Args {
            dir: PathBuf::from("."),
            index: String::from("index.html"),
            not_found: None,
            port: 7878,
            addr: String::from("localhost"),
        }
    }

    pub fn parse(&mut self) -> Result<(), &str> {
        self.store();
        self.verify()
    }

    fn verify(&self) -> Result<(), &str> {
        // Dir check
        if !self.dir.is_dir() {
            return Err("Directory does not exist");
        };
        // Index check
        if !self.dir.join(&self.index).is_file() {
            return Err("Index file does not exist");
        };
        // 404 check
        if !self.not_found.is_none() && !self.dir.join(self.not_found.as_ref().unwrap()).is_file() {
            return Err("404 file does not exist");
        };

        Ok(())
    }

    fn store(&mut self) {
        let mut parser = ArgumentParser::new();
        parser.set_description("Server for yew and yew-router apps");

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
        parser.refer(&mut self.addr).add_option(
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
