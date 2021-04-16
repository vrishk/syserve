use argparse::{ArgumentParser, Store};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Args {
    pub dir: PathBuf,
    pub index: String,
}

impl Args {
    pub fn new() -> Args {
        Args {
            dir: PathBuf::from("."),
            index: String::from("index.html"),
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

        Ok(())
    }

    fn store(&mut self) {
        let mut parser = ArgumentParser::new();
        parser.set_description("Server for yew and yew-router apps");

        parser.refer(&mut self.dir).add_option(
            &["-d", "--dir"],
            Store,
            "Directory to serve (must contain index.html)",
        );
        parser.refer(&mut self.index).add_option(
            &["-i", "--index"],
            Store,
            "Index file to be served",
        );

        parser.parse_args_or_exit();
    }
}
