use reqwest;
use log;

use crate::client::request;
use crate::model::member;

// retrieve member GET
pub async fn member(
    client: &reqwest::Client,
    id: &str,
) -> Result<member::Member, Box<dyn std::error::Error>> {
    log::debug!("retrieving member for {id}");

    // retrieve member and deser
    let member = request::request(client, "members", &format!("/{id}"))
        .await?
        .json::<member::Member>()
        .await?;

    log::debug!("member retrieved for {id}");
    Ok(member)
}

// retrieve members LIST
pub async fn members(
    client: &reqwest::Client,
) -> Result<member::Members, Box<dyn std::error::Error>> {
    log::debug!("retrieving members");

    // retrieve members and deser
    let members = request::request(client, "members", "?limit=100")
        .await?
        .json::<member::Members>()
        .await?;

    log::debug!("members retrieved");
    Ok(members)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member() {
        let test = async {
            assert_eq!(
                member(&reqwest::Client::new(), "1")
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "member retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_members() {
        let test = async {
            assert_eq!(
                members(&reqwest::Client::new())
                    .await
                    .unwrap_err()
                    .to_string(),
                "error decoding response body",
                "member retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
