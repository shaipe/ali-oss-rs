#[macro_use] extern crate lazy_static;

extern crate reqwest;
extern crate md5;

mod consts;
pub mod config;
mod client;
mod base64;
mod utils;
mod bucket;
pub mod service;
// mod md5;

// pub use config::OSSConfig;
pub use client::*;
pub use md5::*;
pub use utils::*;