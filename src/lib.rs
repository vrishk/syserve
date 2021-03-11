use actix_web::{middleware, App, HttpServer};
use argparse::{ArgumentParser, Store};

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

pub async fn server(args: Args) -> std::io::Result<()> {
    // Constants
    let addr: (&str, u16) = ("0.0.0.0", 8000);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/", &args.dir).index_file("index.html"))
            .default_service(actix_files::Files::new(
                "/",
                format!("{}/{}", args.dir, "index.html"),
            ))
    })
    .bind(addr)?
    .workers(1)
    .run()
    .await
}
