use saxobank_rs::client::SaxoClient;

use clap::{arg, command};
use std::error::Error;

fn get_token() -> String
{
    let matches = command!()
        .arg(arg!(
            -t --token <TOKEN> ... "OpenAPI token to run example with"
        ).required(true))
        .get_matches();

    matches.get_one::<String>("token").unwrap().clone()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let client = SaxoClient::new_sim(&get_token())?;
    let resp = client.get_ref_exchanges().await?;
    println!("{:?}", &resp);

    let next_resp = client.get_next(&resp).await?;
    println!("{:?}", &next_resp);

    Ok(())
}