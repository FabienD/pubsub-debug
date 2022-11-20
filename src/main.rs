use dotenv::dotenv;
use std::env;

mod cli;

use cli::Cli;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Get environment variables
    let project_id = env::var("PUBSUB_PROJECT_ID").expect("PUBSUB_PROJECT_ID must be set");
    let endpoint = env::var("PUBSUB_EMULATOR_HOST").expect("PUBSUB_EMULATOR_HOST must be set");
   
    // Execute cli command
    if let Err(err) = Cli::run(project_id, endpoint) {
        println!("Command exection error: {}", err);
    }
}
