use argparse::{ArgumentParser, Store};
use std::path::PathBuf;
use warp::Filter;

#[derive(Clone)]
pub struct Args {
    dir: String,
    index: String,
}

impl Args {
    pub fn new() -> Args {
        Args {
            dir: String::from("."),
            index: String::from("index.html"),
        }
    }

    pub fn parse(&mut self) -> Result<(), &str> {
        self.store();
        self.verify()
    }

    fn verify(&self) -> Result<(), &str> {
        // Dir check
        if !PathBuf::from(&self.dir).is_dir() {
            return Err("Directory does not exist");
        };
        // Index check
        if !PathBuf::from(&self.dir).join(&self.index).is_file() {
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

pub async fn server(args: Args) {
    let localhost = [0, 0, 0, 0];
    let port = 8000;
    let addr = (localhost, port);

    let assets = warp::get()
        .and(warp::fs::dir(args.dir.clone()))
        .and(warp::path::end())
        .boxed();

    let files = assets.or(warp::get()
        .and(warp::fs::file(PathBuf::from(args.dir).join(args.index)))
        .boxed());

    warp::serve(warp::any().and(files).boxed()).run(addr).await;
}
