pub mod addon;
pub mod backup;
pub mod cache;
pub mod catalog;
pub mod config;
pub mod curse_api;
pub mod error;
pub mod fs;
pub mod murmur2;
pub mod network;
pub mod parse;
#[cfg(feature = "gui")]
pub mod theme;
pub mod tukui_api;
pub mod utility;
pub mod wowi_api;

use crate::error::ClientError;

pub type Result<T> = std::result::Result<T, ClientError>;
