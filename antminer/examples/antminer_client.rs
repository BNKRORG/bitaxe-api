use antminer_api::client::AntminerClient;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("http://192.168.1.100")?;
    let client = AntminerClient::new(url, "root", "root")?;

    let info = client.system_info().await?;
    let summary = client.summary().await?;
    println!("{info:#?}");
    println!("{summary:#?}");

    Ok(())
}
