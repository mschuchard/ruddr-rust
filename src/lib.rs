pub mod client {
    pub mod client;
    mod request;
}
pub mod interface {
    pub mod member;
    pub mod project;
    pub mod time;
}
pub(crate) mod model {
    pub(crate) mod client;
    pub(crate) mod member;
    pub(crate) mod project;
    pub(crate) mod time;
    pub mod types;
}
