//! This is the code from the first part of the tutorial
use signalk::signalk::full::V1FullFormat;
use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_url = "https://demo.signalk.org/signalk/v1/api/";
    println!("Connect and get data from: {}", api_url);

    let response = reqwest::get(api_url).await?;

    let sk_data: V1FullFormat = response.json().await?;

    if let Some(self_vessel) = sk_data.get_self() {
        if let Some(ref nav) = self_vessel.navigation {
            if let Some(ref pos) = nav.position {
                print!("Position: lat {} long {}",
                       pos.value.latitude, pos.value.longitude);
            }
        }
    }
    Ok(())
}
