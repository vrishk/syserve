use syserve::cli::Args;
use syserve::server::serve;

#[macro_use]
extern crate log;

use env_logger::Env;

fn main() {
    let env = Env::default()
        .filter_or("SYSERVE_LOG_LEVEL", "trace")
        .write_style_or("SYSERVE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let mut args = Args::new();

    if let Err(s) = args.parse() {
        eprintln!("Error parsing arguments: {}", s);
        std::process::exit(1);
    }

    info!("Starting server...");

    serve(args);
}
