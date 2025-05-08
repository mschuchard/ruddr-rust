pub mod client {
    pub mod client;
    mod request;
}
pub mod interface {
    pub mod allocation;
    pub mod customer;
    pub mod member;
    pub mod project;
    pub mod time;
}
pub mod model {
    pub(crate) mod allocation;
    pub(crate) mod customer;
    pub(crate) mod member;
    pub(crate) mod project;
    pub mod reader;
    pub(crate) mod time;
    pub mod types;
}
