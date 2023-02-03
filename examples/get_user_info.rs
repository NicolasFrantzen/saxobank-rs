use saxobank_rs::client::OpenAPIClient;

use std::error::Error;

const TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let client = OpenAPIClient::new_sim(TOKEN);

    println!("{:?}", client.get_user_info().await?);
    Ok(())
}