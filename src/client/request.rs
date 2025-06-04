//! # Request
//!
//! `client::request` consists of functions for constructing and executing requests against the Ruddr API.
use log;

// request struct for composing request structures
#[derive(Debug)]
pub(super) struct Request {
    pub(super) url: String,
}

impl Request {
    // request constructor with endpoint and params
    pub(super) fn new(endpoint: &str, params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // validate endpoint is not empty
        if endpoint.is_empty() {
            log::error!("endpoint '{endpoint}' is empty");
            return Err(Box::from("invalid empty endpoint"));
        }

        log::debug!("request endpoint is {endpoint} and params is {params}");

        // prefix params with "?" char if they are not empty
        if params.is_empty() {
            Ok(Self {
                url: format!("https://www.ruddr.io/api/workspace/{endpoint}"),
            })
        } else {
            Ok(Self {
                url: format!("https://www.ruddr.io/api/workspace/{endpoint}?{params}"),
            })
        }
    }

    // execute get request with client
    pub(super) async fn get(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        // execute request and receive response
        log::debug!("initiating GET request at {}", self.url);
        let response = client.get(self.url).send().await?;

        log::debug!("response received for GET request");
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_new() {
        assert_eq!(
            Request::new("endpoint", "params")
                .expect("request could not be constructed")
                .url,
            String::from("https://www.ruddr.io/api/workspace/endpoint?params")
        );
        assert_eq!(
            Request::new("endpoint", "")
                .expect("request could not be constructed")
                .url,
            String::from("https://www.ruddr.io/api/workspace/endpoint")
        )
    }

    #[test]
    fn test_request_get() {
        let test = async {
            let client = reqwest::Client::builder()
                .build()
                .expect("client with env token could not be constructed");
            let request = Request::new("projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c", "")
                .expect("request could not be constructed");
            let response = request
                .get(&client)
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
