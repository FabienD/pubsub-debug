use eyre::Result;
use google_cloud_gax::cancel::CancellationToken;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use std::str;

use crate::client;

pub async fn subscribe(project_id: String, endpoint: String, topic: String) -> Result<()> {
    let client = client::new(project_id, endpoint).await.unwrap();

    // Create topic.
    let topic = client.topic(&topic);

    if !topic.exists(None, None).await? {
        println!("Topic does not exist.");
        topic.create(None, None, None).await?;
    }

    // Configure subscription.
    let mut config = SubscriptionConfig::default();
    // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
    config.enable_message_ordering = true;

    // Create subscription
    let subscription = client.subscription("debug-subscription");
    if !subscription.exists(None, None).await? {
        println!("Subscription does not exist.");
        subscription
            .create(topic.fully_qualified_name(), config, None, None)
            .await?;
    }

    // Token for cancel.
    let cancel = CancellationToken::new();

    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.
    let _result = subscription
        .receive(
            |message, _cancel| async move {
                // Handle data.
                let data = message.message.data.as_slice();

                let s = match str::from_utf8(data) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                println!("Received message: {:?}", s);

                // Ack or Nack message.
                let _ack = message.ack().await;
            },
            cancel.clone(),
            None,
        )
        .await;

    // Delete subscription if needed.
    let _delete = subscription.delete(None, None).await;

    Ok(())
}
