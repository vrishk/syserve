use argparse::{ArgumentParser, Store};
use std::path::PathBuf;
use warp::Filter;

#[derive(Clone)]
pub struct Args {
    dir: String,
}

impl Args {
    pub fn new() -> Args {
        Args {
            dir: String::from("."),
        }
    }

    pub fn parse(&mut self) {
        let mut parser = ArgumentParser::new();
        parser.set_description("Server for yew and yew-router apps");

        parser.refer(&mut self.dir).add_option(
            &["-d", "--dir"],
            Store,
            "Directory to serve (must contain index.html)",
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
        .and(warp::fs::file(PathBuf::from(args.dir).join("index.html")))
        .boxed());

    warp::serve(warp::any().and(files).boxed()).run(addr).await;
}
