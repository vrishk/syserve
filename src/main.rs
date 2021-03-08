use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    // Constants
    const ADDR: (&str, u16) = ("0.0.0.0", 8000);

    let args: Vec<String> = std::env::args().collect();
    let dir = args[1].clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/", &dir).index_file("index.html"))
            .default_service(actix_files::Files::new(
                "/",
                format!("{}/{}", dir, "index.html"),
            ))
    })
    .bind(ADDR)?
    .workers(1)
    .run()
    .await
}
