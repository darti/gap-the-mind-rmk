use clap::{Command, Parser, Subcommand};

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
        #[arg(short, long, default_value_t = String::from("10.11.99.1"))]
        ip: String,

        /// ssh user
        #[arg(short, long, default_value_t = String::from("root"))]
        user: String,

        #[arg(short, long)]
        password: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Sshetup { ip, user, password }) => todo!(),
        one => todo!(),
    }

    Ok(())
}
