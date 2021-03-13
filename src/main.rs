use syserve::{server, Args};

#[tokio::main]
async fn main() {
    let mut args = Args::new();

    if let Err(s) = args.parse() {
        eprintln!("Error parsing arguments: {}", s);
        std::process::exit(1);
    }

    server(args).await;
}
