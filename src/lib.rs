pub mod client {
    pub mod client;
    mod request;
}
pub mod interface {
    pub mod customer;
    pub mod member;
    pub mod project;
    pub mod read;
    pub mod time;
}
pub(crate) mod model {
    pub(crate) mod customer;
    pub(crate) mod member;
    pub(crate) mod project;
    pub(crate) mod time;
    pub mod types;
}
