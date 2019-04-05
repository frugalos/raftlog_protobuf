//! Encoders and decoders of [Protocol Buffers] messages for the [`raftlog`]'s constituents.
//!
//! [`raftlog`]: https://github.com/frugalos/raftlog
//! [Protocol Buffers]: https://developers.google.com/protocol-buffers/
#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
extern crate bytecodec;
extern crate protobuf_codec;
extern crate raftlog;
#[macro_use]
extern crate trackable;

#[macro_use]
mod macros;

pub mod log;
pub mod message;
pub mod state;
