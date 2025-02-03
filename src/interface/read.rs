//! # Read
//!
//! `interface::read` consists of a function for Read operation interfacing with the Ruddr endpoints.
use log;

use crate::client::client;
use crate::model::types;
use serde::de;

/// Retrieves a specific Ruddr generic object by id, and deserializes it to the corresponding struct.
/// This is public only to interface at the moment, but is abstract enough that assistance is super helpful to future me, and so documentation exists.
/// ```ignore
/// let deser_response = read::<project::Project>(
///     &client,
///     "projects",
///     types::UUID::from("095e0780-48bf-472c-8deb-2fc3ebc7d90c"),
///     project::Project,
///     "project",
/// ).await?;
/// ```
pub(super) async fn read<M: de::DeserializeOwned>(
    client: &client::Client,
    endpoint: &str,
    id: types::UUID,
    desc: &str,
) -> Result<M, Box<dyn std::error::Error>> {
    log::debug!("retrieving {desc} for {id}");

    // retrieve object and deser
    let deser_response = client
        .execute(endpoint, &format!("/{id}"))
        .await?
        .json::<M>()
        .await?;

    log::debug!("{desc} retrieved for {id}");
    Ok(deser_response)
}
