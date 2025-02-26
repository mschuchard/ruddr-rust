//! # Member
//!
//! `interface::member` consists of functions for interfacing with the Ruddr Member endpoint.
use crate::client::client;
use crate::model::member;
use crate::model::types;

/// Retrieves a specific Ruddr Member object by id, and deserializes it to the corresponding struct.
/// ```ignore
/// let member = member(&client, types::UUID::from("3f3df320-dd95-4a42-8eae-99243fb2ea86")).await?;
/// ```
pub async fn member(
    client: &client::Client,
    id: types::UUID,
) -> Result<member::Member, Box<dyn std::error::Error>> {
    // retrieve member
    Ok(client
        .read::<member::Member>("members", &format!("/{id}"))
        .await?)
}

/// Retrieves all Ruddr Workspace Member objects, and deserializes it to the corresponding vector of structs.
/// ```ignore
/// let members = members(&client).await?;
/// ```
pub async fn members(
    client: &client::Client,
) -> Result<member::Members, Box<dyn std::error::Error>> {
    // retrieve members
    Ok(client
        .read::<member::Members>("members", "?limit=100")
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                member(
                    &client,
                    types::UUID::from("3f3df320-dd95-4a42-8eae-99243fb2ea86")
                )
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
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                members(&client).await.unwrap_err().to_string(),
                "error decoding response body",
                "members retrieval did not fail on json decoding",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
