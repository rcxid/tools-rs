use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// s3 tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// s3 bucket
    #[arg(short, long)]
    bucket: String,

    /// file key
    #[arg(short, long)]
    key: String,

    /// endpoint
    #[arg(short, long)]
    endpoint: String,

    /// region
    #[arg(short, long)]
    region: String,

    /// access_key
    #[arg(short, long)]
    access_key: String,

    /// secret_key
    #[arg(short, long)]
    secret_key: String,

    /// file destination
    #[arg(short, long)]
    destination: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let credentials = Credentials::new(
        args.access_key.as_str(),
        args.secret_key.as_str(),
        None,
        None,
        "aws-sdk-s3",
    );
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(args.endpoint.as_str())
        .region(Region::new(args.region))
        .credentials_provider(credentials)
        .load()
        .await;
    let client = aws_sdk_s3::Client::new(&config);

    let mut file = File::create(args.destination.as_str()).await?;

    let mut object = client
        .get_object()
        .bucket(args.bucket)
        .key(args.key)
        .send()
        .await?;

    while let Some(bytes) = object.body.try_next().await? {
        file.write_all(&bytes).await?;
    }

    Ok(())
}
