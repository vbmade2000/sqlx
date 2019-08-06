use crate::backend::Backend;

mod connection;

pub use connection::Connection;

pub(crate) mod protocol;

pub mod types;

pub struct Postgres;

impl Backend for Postgres {
    type RawRow = protocol::DataRow;
    type TypeMetadata = types::TypeMetadata;
}
