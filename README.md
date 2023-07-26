# MQTT Publisher Example App

A simple proof-of-concept app that publishes messages to the public MQTT server at `tcp://broker.emqx.io:1883`

This app works with in conjunction with a corresponding [subscriber app](https://github.com/lighthouse-no/demo-mqtt-sub).

![MQTT Demo Pub/Sub App](./img/architecture.png)

## Usage

1. Ensure that the corresponding subscriber app has first been pushed to Cloud Foundry and is running
1. Compile with `cargo build --release`
1. Start publisher `./target/release/mqtt-pub`

The publisher sends 5 messages on the alternating MQTT topics `"rust/mqtt"`, and `"rust/test"`, then terminates.
