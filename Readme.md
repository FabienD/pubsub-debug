# A litte cli tool to push and subscribe to GPC PubSub emulator

## Build the cli

This require Rust and Cargo to be build.

```bash
cargo build --release
```

Copy the binary `target/release/pubsub-debug` to your path.

The cli use .env to get the emulator host, port and the project id.

You can create a .env file in the root of the project by copying the .env.dist file.
You can also set the environment variables directly before running the cli.

A docker-compose file is provided to run the emulator and use the same .env file.

## Publish a message

```bash
pubsub-debug publish --topic=topic-name --message="Hello World"
```

## Subscribre and receive message

```bash
pubsub-debug subscribre --topic=topic-name
```