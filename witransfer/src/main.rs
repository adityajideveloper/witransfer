use clap::{Parser, Subcommand};
use num_cpus;
use witransfer::networking::discover;

#[derive(Parser)]
#[command(name = "WiTransfer")]
#[command(about = "A command-line tool for transferring files over Wi-Fi.")]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Discover other devices on the same network
    Discover {
        /// Port on which you will be receiving
        #[arg(default_value_t = 54321)]
        port: u16,
    },
    Send {
        path: std::path::PathBuf,
    },
    Cores,
}
fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Discover { port } => {
            println!("Discovering... on port -> {port}");
            discover(*port)
        }
        Commands::Cores => {
            println!("This machines has {} cores/threads.", { num_cpus::get() })
        }
        _ => println!("Doing something else."),
    }
}
