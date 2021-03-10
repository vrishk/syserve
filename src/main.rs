use actix_web::{middleware, App, HttpServer};
use argparse::{ArgumentParser, Store};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // Constants
    let addr: (&str, u16) = ("0.0.0.0", 8000);
    let mut dir = String::from(".");

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Server for yew and yew-router apps");

        parser.refer(&mut dir).add_option(
            &["-d", "--dir"],
            Store,
            "Directory to serve containing index.html",
        );
        parser.parse_args_or_exit();
    }

    println!("{:?}", dir);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/", &dir).index_file("index.html"))
            .default_service(actix_files::Files::new(
                "/",
                format!("{}/{}", dir, "index.html"),
            ))
    })
    .bind(addr)?
    .workers(1)
    .run()
    .await
}
