use saxobank_rs::client::OpenAPIClient;

const TOKEN: &str = "";

#[ignore]
#[tokio::test]
async fn test_get_user_info() {
    let client = OpenAPIClient::new_sim(TOKEN);

    let body = client.get_user_info().await;

    #[cfg(debug_assertions)]
    dbg!(&body);

    assert!(body.is_ok());
}

#[ignore]
#[tokio::test]
async fn test_get_client_info() {
    let client = OpenAPIClient::new_sim(TOKEN);

    let body = client.get_client_info().await;

    #[cfg(debug_assertions)]
    dbg!(&body);

    assert!(body.is_ok());
}
