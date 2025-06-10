use super::*;
use crate::model::project;

#[test]
fn test_client_new() {
    let test = async {
        let client = Client::new(Some("abcdefghi123456789"))
            .expect("client with token could not be constructed");
        println!("client: {:?}", client)
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_client_new_env() {
    // unsafe because tests are multi-threaded
    unsafe {
        std::env::set_var("RUDDR_TOKEN", "abcdefghi123456789");
    }
    let test = async {
        let client = Client::new(None).expect("client with env token could not be constructed");
        println!("client: {:?}", client)
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);

    // unsafe because tests are multi-threaded
    unsafe {
        std::env::remove_var("RUDDR_TOKEN");
    }
    let test = async {
        assert_eq!(
            Client::new(None).unwrap_err().to_string(),
            "ruddr api token was not input through code or RUDDR_TOKEN environment variable",
            "attempted client build without token did not error expectedly",
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_client_read() {
    let test = async {
        let client = Client::new(Some("abcdefghi123456789"))
            .expect("client with env token could not be constructed");
        assert_eq!(
            client
                .read::<project::Project>("projects/095e0780-48bf-472c-8deb-2fc3ebc7d90c", "",)
                .await
                .unwrap_err()
                .to_string(),
            "client read response failed",
            "read did not fail on auth"
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_client_list() {
    let test = async {
        let client = Client::new(Some("abcdefghi123456789"))
            .expect("client with env token could not be constructed");
        assert_eq!(
            client
                .read::<project::Project>("projects", "limit=100")
                .await
                .unwrap_err()
                .to_string(),
            "client read response failed",
            "list did not fail on auth"
        )
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
