use bitaxe_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("http://192.168.1.100")?;
    let client = BitaxeClient::new(url)?;

    let info = client.system_info().await?;
    println!("{info:#?}");

    Ok(())
}
