use syserve::{server, Args};

#[tokio::main]
async fn main() {
    let mut args = Args::new();
    args.parse();

    server(args).await;
}
