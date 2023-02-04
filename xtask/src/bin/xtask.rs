use clap::{Parser, Subcommand};
use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use xtask::ssh::init_connect;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// setup ssh key
    Sshetup {
        /// remarkable ip address, either newtork over usb or wifi
        #[arg(long, default_value_t = String::from("10.11.99.1"))]
        host: String,

        #[arg(long, default_value_t = 22)]
        port: u16,

        /// ssh user
        #[arg(short, long, default_value_t = String::from("root"))]
        user: String,

        #[arg(short, long)]
        password: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Sshetup {
            host,
            port,
            user,
            password,
        }) => sshetup(host, port, user, password),
        None => todo!(),
    }
    .await
}

async fn sshetup(
    host: String,
    port: u16,
    user: String,
    password: Option<String>,
) -> anyhow::Result<()> {
    info!("Sshetup: {}@{}:{}", user, host, port);

    let password = password.unwrap_or_else(|| {
        rpassword::prompt_password(format!("password for{}@{}: ", user, host)).unwrap()
    });

    let conn = init_connect(&host, port, &user, &password).await;

    Ok(())
}
