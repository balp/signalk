//! This is the code from the first part of the tutorial
use reqwest::Error;
use signalk::V1FullFormat;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_url = "https://demo.signalk.org/signalk/v1/api/";
    println!("Connect and get data from: {}", api_url);

    let response = reqwest::get(api_url).await?;

    let sk_data: V1FullFormat = response.json().await?;

    if let Some(self_vessel) = sk_data.get_self() {
        if let Some(ref nav) = self_vessel.navigation {
            if let Some(ref pos) = nav.position {
                if let Some(ref pos_value) = pos.value {
                    print!(
                        "Position: lat {} long {}",
                        pos_value.latitude, pos_value.longitude
                    );
                }
            }
        }
    }
    Ok(())
}
