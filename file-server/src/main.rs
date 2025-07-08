use clap::Parser;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

/// Simple file server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file server path
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let router = Router::with_path("{*path}").get(
        StaticDir::new([args.path])
            .include_dot_files(false)
            .auto_list(true),
    );
    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}
