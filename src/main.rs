use syserve::{server, Args};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let mut args = Args::new();
    args.parse();

    server(args).await
}
