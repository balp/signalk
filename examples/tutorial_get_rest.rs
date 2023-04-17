
use signalk::signalk::full::V1FullFormat;
use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_url = "https://demo.signalk.org/signalk/v1/api/";
    println!("url: {}", api_url);

    let response = reqwest::get(api_url).await?;

    let sk_data: V1FullFormat = response.json().await?;

    println!("Got: {:?}", sk_data);
    Ok(())
}
