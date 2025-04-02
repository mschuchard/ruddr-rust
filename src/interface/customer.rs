//! # Customer
//!
//! `interface::customer` consists of functions for interfacing with the Ruddr Client endpoint.
use crate::client::client;
use crate::model::customer;
use crate::model::types;

/// Retrieves a specific Ruddr Client object by id, and deserializes it to the corresponding struct.
/// ```ignore
/// let customer = customer(&client, Some(types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0"))).await?;
/// ```
pub async fn customer(
    client: &client::Client,
    id: types::UUID,
) -> Result<customer::Client, Box<dyn std::error::Error>> {
    // retrieve client
    Ok(client
        .read::<customer::Client>("clients", &format!("/{id}"))
        .await?)
}

/// Retrieves all Ruddr Client objects by filters, and deserializes it to the corresponding vector of structs.
/// ```ignore
/// let customers = customers(
///     &client,
///     Some("JOE")),
/// ).await?;
/// ```
pub async fn customers(
    client: &client::Client,
    code: Option<&str>,
) -> Result<customer::Clients, Box<dyn std::error::Error>> {
    // initialize params
    let mut params = String::from("?limit=100");

    // optional parameters for LIST
    if code.is_some() {
        params = format!("{params}&code={}", code.unwrap())
    }

    // retrieve clients
    Ok(client.read::<customer::Clients>("clients", &params).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                customer(
                    &client,
                    types::UUID::from("4cacdf11-71d1-4fbb-90ee-b091803581b0")
                )
                .await
                .unwrap_err()
                .to_string(),
                "client read response failed",
                "client retrieval did not fail on auth",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }

    #[test]
    fn test_customers() {
        let test = async {
            let client = client::Client::new(Some("abcdefghi123456789"))
                .await
                .expect("client with token could not be constructed");
            assert_eq!(
                customers(&client, Some("JOE"))
                    .await
                    .unwrap_err()
                    .to_string(),
                "client read response failed",
                "clients retrieval did not fail on auth",
            )
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(test);
    }
}
