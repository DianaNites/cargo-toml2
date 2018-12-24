#![warn(clippy::all)]
#![warn(clippy::pedantic)]
mod config;
mod manifest;
pub use self::{config::*, manifest::*};
