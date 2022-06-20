//! Email addresses

#[cfg(feature = "serde")]
mod serde;

mod envelope;
mod types;
#[cfg(feature = "delivery-status-notification")]
mod dsn_config;

pub use self::{
    envelope::Envelope,
    types::{Address, AddressError},
};

#[cfg(feature = "delivery-status-notification")]
pub use self::dsn_config::{DsnConfig, Ret, Action};
