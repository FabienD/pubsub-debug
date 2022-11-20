use eyre::Result;

use crate::client;

pub async fn publish(project_id: String, endpoint: String, topic: String, message: String) -> Result<()> 
{   
    let client = client::new(project_id, endpoint).await.unwrap();

    // Create topic.
    let topic = client.topic(&topic);

    if !topic.exists(None, None).await? {
        topic.create(None, None, None).await?;
    }
    
    Ok(())
}