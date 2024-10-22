use log;
use std::env;

use reqwest;

// initialize client
pub async fn client_build(
    token: Option<&str>,
) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    // initialize headers and establish json encoding
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    // determine authentication input method
    let bearer_token = match token {
        Some(token) => format!("Bearer {token}"),
        None => match env::var("RUDDR_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err(Box::from(
                "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
            )),
        },
    };
    // establish authentication and mark as sensitive
    let mut auth_token = reqwest::header::HeaderValue::from_str(&bearer_token)?;
    auth_token.set_sensitive(true);
    headers.insert(reqwest::header::AUTHORIZATION, auth_token);

    log::debug!("built headers are {:?}", headers);

    // build client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    log::debug!("built client is {:?}", client);
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_build() {
        let test = async {
            let client = client_build(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            println!("client: {:?}", client)
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_client_build_env() {
        let test = async {
            // supposed to only be safe in single-thread
            std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789");
            let client = client_build(None)
                .await
                .expect("client with env token could not be constructed");
            println!("client: {:?}", client)
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_client_build_env_error() {
        let test = async {
            // supposed to only be safe in single-thread
            std::env::remove_var("RUDDR_TOKEN");
            assert_eq!(
                client_build(None).await.unwrap_err().to_string(),
                "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
                "attempted client build without token did not error expectedly",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
