use clap::{Parser, Subcommand};
use eyre::Result;

use crate::publisher;
use crate::subscriber;

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
    pub async fn run(project_id: String, endpoint: String) -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Subscribe { topic } => {
                subscriber::subscribe(project_id, endpoint, topic.to_string()).await?;
            }
            Commands::Publish { topic, message } => {
                publisher::publish(project_id, endpoint, topic.to_string(), message.to_string())
                    .await?;
            }
        }

        Ok(())
    }
}
