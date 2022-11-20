
    // Create pubsub client.
    // The default project is determined by credentials.
    // - If the GOOGLE_APPLICATION_CREDENTIALS is specified the project_id is from credentials.
    // - If the server is running on GCP the project_id is from metadata server
    // - If the PUBSUB_EMULATOR_HOST is specified the project_id is 'local-project'
    let config = ClientConfig { 
        project_id: Some("my-project-id".to_string()),
        pool_size: Some(4),
        endpoint: "http://localhost:8085".to_string(),
    };

    let client = Client::new(config).await.unwrap();

    // Create topic.
    let topic = client.topic("test-topic");
    if !topic.exists(None, None).await? {
        topic.create(None, None, None).await?;
    }
    
    // Start publisher.
    let mut publisher = topic.new_publisher(None);

    // Publish message.
    let tasks : Vec<JoinHandle<Result<String,Status>>> = (0..10).into_iter().map(|_i| {
        let publisher = publisher.clone();
        tokio::spawn(async move {
            let mut msg = PubsubMessage::default();
            msg.data = "abc".into();
            // Set ordering_key if needed (https://cloud.google.com/pubsub/docs/ordering)
            // msg.ordering_key = "order".into();

            // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
            let awaiter = publisher.publish(msg).await;

            // The get method blocks until a server-generated ID or an error is returned for the published message.
            awaiter.get(None).await
        })
    }).collect();

    // Wait for all publish task finish
    for task in tasks {
        let message_id = task.await.unwrap()?;
        println!("Message published: {}", message_id);
    }

    // Wait for publishers in topic finish.
    publisher.shutdown().await;

    Ok(())