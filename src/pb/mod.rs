use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform ::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;
mod abi; // 声明 abi.rs
pub use abi::*;

impl ImageSpec {
    pub fn new(spaces: Vec<Spec>) -> Self {
        Self { specs }
    }
}
