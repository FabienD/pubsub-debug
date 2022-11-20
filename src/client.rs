use eyre::Result;
use google_cloud_pubsub::client::{Client, ClientConfig};

pub async fn new(project_id: String, endpoint: String) -> Result<Client> {
    let config: ClientConfig = ClientConfig {
        project_id: Some(project_id),
        pool_size: Some(4),
        endpoint,
    };

    let client = Client::new(config).await.unwrap();

    Ok(client)
}
