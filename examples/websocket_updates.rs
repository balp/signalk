use serde_json;
use signalk::{V1DeltaFormat, V1FullFormat, V1Hello};
use tungstenite::connect;

fn main() {
    env_logger::init();
    let mut data = V1FullFormat::default();
    let (mut socket, _response) =
        connect("ws://127.0.0.1:3000/signalk/v1/stream").expect("Can't connect");

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
        // Get the current battery voltage, and display it
        let depth_below_keel =
            data.get_f64_for_path("self.environment.depth.belowKeel".to_string());
        log::info!("Depth: {:?} from {:?}", depth_below_keel, data);
        if let Ok(voltage) = depth_below_keel {
            println!("Current depth is {}", voltage)
        }
    }
}
