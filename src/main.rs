use clap::{Parser, Subcommand};

pub mod commands;
pub mod devices;
pub mod ui;
pub mod utils;

#[derive(Parser, Debug)]
#[clap(
    author = "Kadir & Yo Contributors",
    about = "(Y)ammy (O)utputs for ADB - An Helper for ADB",
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Devices,
    Version,
    Connect { host: String },
    Disconnect { host: String },
    Push { from: String, to: String },
    Pull { from: String, to: String },
    Install { app_path: String },
    Shell,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    match args.command {
        Commands::Version => commands::version(),
        Commands::Devices => commands::devices(),
        Commands::Connect { host } => commands::connect(host),
        Commands::Disconnect { host } => commands::disconnect(host),
        Commands::Push { from, to } => commands::push(from, to),
        Commands::Pull { from, to } => commands::pull(from, to),
        Commands::Install { app_path } => commands::install(app_path),
        Commands::Shell => commands::shell(),
    };
}
