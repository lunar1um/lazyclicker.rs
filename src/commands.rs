use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lazyclicker")]
#[command(about = "lazy autoclicker or holder", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a blank config file
    Init,

    /// Start the auto clicker
    Start {
        /// Name of the profile
        name: String,

        #[arg(long, hide = true, default_value_t = false)]
        __run: bool,
    },

    /// Stop the auto clicker
    Stop { name: String },

    /// List all available profiles
    List,
}