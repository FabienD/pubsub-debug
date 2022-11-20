use clap::{Parser, Subcommand};
use eyre::{Result};

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Helper tools to publish message in pubsub or subscribe to a topic."
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// "Publish message to a topic."
    Publish {
        /// The topic to publish to.
        #[clap(value_parser)]
        topic: String,
        /// The message to publish.
        #[clap(value_parser)]
        message: String,
    },
    /// "Subscribe to a topic
    Subscribe {
        /// The topic to subscribe to.
        topic: String,
    },
}

impl Cli {
    pub fn run(project_id: String, endpoint: String) -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Subscribe { topic } => {
                println!("Subscribing to topic: {}", topic);
            },
            Commands::Publish { topic, message } => {
                println!("Publishing message {} to topic: {}", message, topic);
            },
        }

        Ok(())
    }
}