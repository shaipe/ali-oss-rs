// Copyright © Shaipe
//! aliyun oss sdk for Rust
//! 

#[macro_use] extern crate lazy_static;

extern crate reqwest;
extern crate md5;

mod consts;
pub mod config;
mod client;
mod base64;
mod utils;
pub mod bucket;
// pub use bucket::put_object;

pub mod service;
// mod md5;

// pub use config::OSSConfig;
// use client::*;
// use md5::*;
// use utils::*;
pub use config::{OSSConfig, set_oss_config};