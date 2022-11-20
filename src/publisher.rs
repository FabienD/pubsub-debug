use eyre::Result;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;

use crate::client;

pub async fn publish(
    project_id: String,
    endpoint: String,
    topic: String,
    message: String,
) -> Result<()> {
    let client = client::new(project_id, endpoint).await.unwrap();

    // Create topic.
    let topic = client.topic(&topic);

    if !topic.exists(None, None).await? {
        println!("Creating topic");
        topic.create(None, None, None).await?;
    }

    // Start publisher.
    let mut publisher = topic.new_publisher(None);

    let mut msg = PubsubMessage::default();
    msg.data = message.into();

    // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
    let awaiter = publisher.publish(msg).await;

    // The get method blocks until a server-generated ID or an error is returned for the published message.
    let message_id = awaiter.get(None).await.unwrap();
    println!("Message ID: {}", message_id);

    publisher.shutdown().await;

    Ok(())
}
