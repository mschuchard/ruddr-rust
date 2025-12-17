pub mod client {
    pub mod client;
    mod request;
}
pub mod interface {
    pub mod allocation;
    pub mod cost;
    pub mod customer;
    pub mod expense_item;
    pub mod expense_report;
    pub mod member;
    pub mod project;
    pub mod role;
    pub mod time;
    pub mod utilization;
}
pub mod model {
    pub(crate) mod allocation;
    pub(crate) mod client;
    pub(crate) mod cost;
    pub mod enums;
    pub(crate) mod expense_item;
    pub(crate) mod expense_report;
    pub(crate) mod member;
    pub(crate) mod project;
    pub(crate) mod role;
    pub(crate) mod shared;
    pub(crate) mod time;
    pub mod types;
    pub(crate) mod utilization;
}
