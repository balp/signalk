use serde_json;
use tungstenite::{connect, Message};

use signalk::{V1DeltaFormat, V1FullFormat, V1Hello, V1Subscribe, V1Subscription};

fn main() {
    // Initialice the env logger
    env_logger::init();
    // Set up the default data storage for full SignalK data
    let mut data = V1FullFormat::default();

    // Connect to SignalK server on local network (annoiii.lan)
    // without any subscribed data. Then send an subscription message for only
    // battery voltage
    let (mut socket, _response) =
        connect("ws://annoiii.lan:3000/signalk/v1/stream?subscribe=none")
            .expect("Can't connect");
    let subscribe = V1Subscribe::builder()
        .context("self".to_string())
        .subscribe(
            V1Subscription::builder()
                .path("electrical.batteries.house.voltage".to_string())
                .period(1000)
                .policy("fixed".to_string())
                .build(),
        )
        .build();
    if let Ok(s) = serde_json::to_string(&subscribe) {
        socket.send(Message::Text(s.into())).unwrap();
    }

    // Loop forever, handling parsing each message
    loop {
        let ws_message = socket.read().expect("Error reading message");
        let raw_json = match ws_message {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!()
            }
        };
        let _parsed: serde_json::Value =
            serde_json::from_str(&raw_json).expect("Can't parse to JSON");

        // The SignalK hello message, will tell us the servers self value, add it to the data
        if let Ok(hello) = serde_json::from_str::<V1Hello>(&raw_json) {
            log::info!("hello: {:?}", hello);
            if let Some(self_id) = hello.self_ {
                data.self_ = self_id;
            }
        }

        // Apply any delta messages to the data storage
        if let Ok(delta) = serde_json::from_str::<V1DeltaFormat>(&raw_json) {
            log::debug!("delta: {:?}", delta);
            data.apply_delta(&delta);
        }

        // Get the current battery voltage, and display it
        let voltage_result =
            data.get_f64_for_path("self.electrical.batteries.house.voltage".to_string());
        log::info!("Voltage: {:?} from {:?}", voltage_result, data);
        if let Ok(voltage) = voltage_result {
            println!("Current voltage is {}", voltage)
        }
    }
}
