mod replay_gk;

use clap::{AppSettings, Parser};

#[derive(Parser, Debug)]
#[clap(about = "The Phala TEE worker app.", version, author)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    #[clap(
        default_value = "ws://localhost:9944",
        long,
        help = "Substrate rpc websocket endpoint."
    )]
    node_uri: String,

    #[clap(
        default_value = "413895",
        long,
        help = "The block number to start to replay at."
    )]
    start_at: u32,

    #[clap(
        default_value = "127.0.0.1:8080",
        long,
        help = "Bind address for local HTTP server."
    )]
    bind_addr: String,

    #[clap(
        default_value = "",
        long,
        help = "The PostgresQL database to store the events."
    )]
    persist_events_to: String,

    #[clap(
        default_value = "0",
        long,
        help = "Assume the give number of block finalized."
    )]
    assume_finalized: u32,

    #[clap(
        default_value = "100000",
        long,
        help = "The number of blocks between two checkpoints. 0 for disabled"
    )]
    checkpoint_interval: u32,

    #[clap(
        long,
        help = "The checkpoint file to restore from. Default is to use the latest checkpoint."
    )]
    restore_from: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    replay_gk::replay(args).await.expect("Failed to run replay");
}
