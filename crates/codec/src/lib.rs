#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
extern crate alloc;
extern crate core;

pub use crate::{
    buffer::{BufferDecoder, BufferEncoder, WritableBuffer},
    empty::EmptyVec,
    encoder::{Encoder, FieldEncoder},
};

mod buffer;
mod empty;
mod encoder;
mod evm;
mod hash;
mod macros;
mod primitive;
mod serde;
#[cfg(test)]
mod tests;
mod tuple;
mod vec;

pub use fluentbase_codec_derive::Codec;
