use syserve::cli::Args;
use syserve::server::serve;

fn main() {
    let mut args = Args::new();

    if let Err(s) = args.parse() {
        eprintln!("Error parsing arguments: {}", s);
        std::process::exit(1);
    }

    serve(args);
}
