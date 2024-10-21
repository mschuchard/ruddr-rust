use reqwest;
use log;

pub(crate) async fn request(
    client: &reqwest::Client,
    endpoint: &str,
    params: &str,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    // establish full url
    let url = format!("https://www.ruddr.io/api/workspace/{endpoint}{params}");

    log::debug!("initiating GET request at {url}");

    // TODO: deser on generic struct like in Go?
    let response = client.get(url).send().await?;

    log::debug!("response received for GET request");
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let test = async {
            let response = request(&reqwest::Client::new(), "members", "/1")
                .await
                .expect("request transmission failed to receive a response");
            println!("response: {:?}", response);
            assert_eq!(
                response.status(),
                401,
                "the response did not return expected 401 status",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
