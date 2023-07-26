use std::{env, process, time::Duration};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT: &str = "rust_publish";
const DFLT_TOPICS: [&str; 2] = ["rust/mqtt", "rust/test"];

// Define the qos
const QOS: i32 = 1;

const MSG: &[u8] = "Hello world! ".as_bytes();

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| DFLT_BROKER.to_string());

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Define the set of options for the connection.
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Create 5 messages and publish them to alternate topics
    for num in 0..5 {
        let topic = DFLT_TOPICS[(num % 2) as usize];
        println!("Publishing message on topic {:?}", topic);

        // In the ASCII collation sequence, digits start at 0x30, so (u8::from(num) | 0x30) is a simple way to
        // convert an unsigned integer to its ASCII character 😎
        let msg = mqtt::Message::new(topic, [MSG, &[(u8::from(num) | 0x30)]].concat(), QOS);
        let tok = cli.publish(msg);

        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
            break;
        }
    }

    // Disconnect from the broker.
    println!("Disconnecting from the broker");
    cli.disconnect(None).unwrap();
}
