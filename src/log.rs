//! Encoders and decoders for the constituents defined in `raftlog::log` module.
use bytecodec::combinator::PreEncode;
use protobuf_codec::field::branch::Branch2;
use protobuf_codec::field::num::{F1, F2, F3, F4, F5};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Oneof, Optional,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{BytesDecoder, BytesEncoder, Uint64Decoder, Uint64Encoder};
use raftlog::log::{LogEntry, LogPosition, LogPrefix};

use state::{ClusterConfigDecoder, ClusterConfigEncoder};

/// Decoder for `LogEntry`.
#[derive(Debug, Default)]
pub struct LogEntryDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, Uint64Decoder>>,
            Optional<
                Oneof<(
                    FieldDecoder<F2, BytesDecoder>,
                    MessageFieldDecoder<F3, ClusterConfigDecoder>,
                )>,
            >,
        )>,
    >,
}
impl_message_decode!(LogEntryDecoder, LogEntry, |t: (u64, _)| {
    let term = t.0.into();
    Ok(match t.1 {
        None => LogEntry::Noop { term },
        Some(Branch2::A(command)) => LogEntry::Command { term, command },
        Some(Branch2::B(config)) => LogEntry::Config { term, config },
    })
});

/// Encoder for `LogEntry`.
#[derive(Debug, Default)]
pub struct LogEntryEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, Uint64Encoder>,
            Optional<
                Oneof<(
                    FieldEncoder<F2, BytesEncoder>,
                    MessageFieldEncoder<F3, PreEncode<ClusterConfigEncoder>>,
                )>,
            >,
        )>,
    >,
}
impl_sized_message_encode!(LogEntryEncoder, LogEntry, |item: Self::Item| match item {
    LogEntry::Noop { term } => (term.as_u64(), None),
    LogEntry::Command { term, command } => (term.as_u64(), Some(Branch2::A(command))),
    LogEntry::Config { term, config } => (term.as_u64(), Some(Branch2::B(config))),
});

/// Decoder for `LogPrefix`.
#[derive(Debug, Default)]
pub struct LogPrefixDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F3, ClusterConfigDecoder>,
            MaybeDefault<FieldDecoder<F4, BytesDecoder>>,
            MessageFieldDecoder<F5, LogPositionDecoder>,
        )>,
    >,
}
impl_message_decode!(LogPrefixDecoder, LogPrefix, |t: (_, _, _)| Ok(LogPrefix {
    config: t.0,
    snapshot: t.1,
    tail: t.2,
}));

/// Encoder for `LogPrefix`.
#[derive(Debug, Default)]
pub struct LogPrefixEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F5, LogPositionEncoder>,
            MessageFieldEncoder<F3, PreEncode<ClusterConfigEncoder>>,
            FieldEncoder<F4, BytesEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(LogPrefixEncoder, LogPrefix, |item: Self::Item| (
    item.tail,
    item.config,
    item.snapshot
));

/// Decoder for `LogPosition`.
#[derive(Debug, Default)]
pub struct LogPositionDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, Uint64Decoder>>,
            MaybeDefault<FieldDecoder<F2, Uint64Decoder>>,
        )>,
    >,
}
impl_message_decode!(LogPositionDecoder, LogPosition, |t: (u64, u64)| Ok(
    LogPosition {
        prev_term: t.0.into(),
        index: t.1.into()
    }
));

/// Encoder for `LogPosition`.
#[derive(Debug, Default)]
pub struct LogPositionEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, Uint64Encoder>,
            FieldEncoder<F2, Uint64Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(LogPositionEncoder, LogPosition, |item: Self::Item| (
    item.prev_term.as_u64(),
    item.index.as_u64()
));
