//! Encoders and decoders for the constituents defined in `raftlog::message` module.
use protobuf_codec::field::num::{F1, F2, F3, F4};
use protobuf_codec::field::{
    FieldDecoder, FieldEncoder, Fields, MaybeDefault, MessageFieldDecoder, MessageFieldEncoder,
    Repeated,
};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    BoolDecoder, BoolEncoder, StringDecoder, StringEncoder, Uint64Decoder, Uint64Encoder,
};
use raftlog::log::{LogEntry, LogSuffix};
use raftlog::message::{
    AppendEntriesCall, AppendEntriesReply, InstallSnapshotCast, MessageHeader, RequestVoteCall,
    RequestVoteReply, SequenceNumber,
};
use raftlog::node::NodeId;

use log::{
    LogEntryDecoder, LogEntryEncoder, LogPositionDecoder, LogPositionEncoder, LogPrefixDecoder,
    LogPrefixEncoder,
};

/// Decoder for `RequestVoteCall` message.
#[derive(Debug, Default)]
pub struct RequestVoteCallDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, HeaderDecoder>,
            MaybeDefault<MessageFieldDecoder<F2, LogPositionDecoder>>,
        )>,
    >,
}
impl_message_decode!(RequestVoteCallDecoder, RequestVoteCall, |t: (_, _)| Ok(
    RequestVoteCall {
        header: t.0,
        log_tail: t.1
    }
));

/// Encoder for `RequestVoteCall` message.
#[derive(Debug, Default)]
pub struct RequestVoteCallEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, HeaderEncoder>,
            MaybeDefault<MessageFieldEncoder<F2, LogPositionEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(
    RequestVoteCallEncoder,
    RequestVoteCall,
    |item: Self::Item| (item.header, item.log_tail)
);

/// Decoder for `RequestVoteReply` message.
#[derive(Debug, Default)]
pub struct RequestVoteReplyDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, HeaderDecoder>,
            MaybeDefault<FieldDecoder<F2, BoolDecoder>>,
        )>,
    >,
}
impl_message_decode!(
    RequestVoteReplyDecoder,
    RequestVoteReply,
    |(header, voted)| Ok(RequestVoteReply { header, voted })
);

/// Encoder for `RequestVoteReply` message.
#[derive(Debug, Default)]
pub struct RequestVoteReplyEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, HeaderEncoder>,
            MaybeDefault<FieldEncoder<F2, BoolEncoder>>,
        )>,
    >,
}
impl_sized_message_encode!(
    RequestVoteReplyEncoder,
    RequestVoteReply,
    |item: Self::Item| (item.header, item.voted)
);

/// Decoder for `AppendEntriesCall` message.
#[derive(Debug, Default)]
pub struct AppendEntriesCallDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, HeaderDecoder>,
            MaybeDefault<FieldDecoder<F2, Uint64Decoder>>,
            MessageFieldDecoder<F3, LogPositionDecoder>,
            Repeated<MessageFieldDecoder<F4, LogEntryDecoder>, Vec<LogEntry>>,
        )>,
    >,
}
impl_message_decode!(
    AppendEntriesCallDecoder,
    AppendEntriesCall,
    |t: (_, u64, _, _)| Ok(AppendEntriesCall {
        header: t.0,
        committed_log_tail: t.1.into(),
        suffix: LogSuffix {
            head: t.2,
            entries: t.3
        }
    })
);

/// Encoder for `AppendEntriesCall` message.
#[derive(Debug, Default)]
pub struct AppendEntriesCallEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, HeaderEncoder>,
            FieldEncoder<F2, Uint64Encoder>,
            MessageFieldEncoder<F3, LogPositionEncoder>,
            Repeated<MessageFieldEncoder<F4, LogEntryEncoder>, Vec<LogEntry>>,
        )>,
    >,
}
impl_message_encode!(
    AppendEntriesCallEncoder,
    AppendEntriesCall,
    |item: Self::Item| (
        item.header,
        item.committed_log_tail.as_u64(),
        item.suffix.head,
        item.suffix.entries
    )
);

/// Decoder for `AppendEntriesReply` message.
#[derive(Debug, Default)]
pub struct AppendEntriesReplyDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, HeaderDecoder>,
            MaybeDefault<MessageFieldDecoder<F2, LogPositionDecoder>>,
            MaybeDefault<FieldDecoder<F3, BoolDecoder>>,
        )>,
    >,
}
impl_message_decode!(
    AppendEntriesReplyDecoder,
    AppendEntriesReply,
    |t: (_, _, _)| Ok(AppendEntriesReply {
        header: t.0,
        log_tail: t.1,
        busy: t.2
    })
);

/// Encoder for `AppendEntriesReply` message.
#[derive(Debug, Default)]
pub struct AppendEntriesReplyEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, HeaderEncoder>,
            MessageFieldEncoder<F2, LogPositionEncoder>,
            FieldEncoder<F3, BoolEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(
    AppendEntriesReplyEncoder,
    AppendEntriesReply,
    |item: Self::Item| (item.header, item.log_tail, item.busy)
);

/// Decoder for `InstallSnapshotCast` message.
#[derive(Debug, Default)]
pub struct InstallSnapshotCastDecoder {
    inner: MessageDecoder<
        Fields<(
            MessageFieldDecoder<F1, HeaderDecoder>,
            MessageFieldDecoder<F2, LogPrefixDecoder>,
        )>,
    >,
}
impl_message_decode!(
    InstallSnapshotCastDecoder,
    InstallSnapshotCast,
    |(header, prefix)| Ok(InstallSnapshotCast { header, prefix })
);

/// Encoder for `InstallSnapshotCast` message.
#[derive(Debug, Default)]
pub struct InstallSnapshotCastEncoder {
    inner: MessageEncoder<
        Fields<(
            MessageFieldEncoder<F1, HeaderEncoder>,
            MessageFieldEncoder<F2, LogPrefixEncoder>,
        )>,
    >,
}
impl_message_encode!(
    InstallSnapshotCastEncoder,
    InstallSnapshotCast,
    |item: Self::Item| (item.header, item.prefix)
);

/// Decoder for `MessageHeader`.
#[derive(Debug, Default)]
pub struct HeaderDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, StringDecoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
            MaybeDefault<FieldDecoder<F3, Uint64Decoder>>,
            MaybeDefault<FieldDecoder<F4, Uint64Decoder>>,
        )>,
    >,
}
impl_message_decode!(HeaderDecoder, MessageHeader, |t: (_, _, _, u64)| Ok(
    MessageHeader {
        sender: NodeId::new(t.0),
        destination: NodeId::new(t.1),
        seq_no: SequenceNumber::new(t.2),
        term: t.3.into()
    }
));

/// Decoder for `MessageHeader`.
#[derive(Debug, Default)]
pub struct HeaderEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, StringEncoder>,
            FieldEncoder<F2, StringEncoder>,
            FieldEncoder<F3, Uint64Encoder>,
            FieldEncoder<F4, Uint64Encoder>,
        )>,
    >,
}
impl_sized_message_encode!(HeaderEncoder, MessageHeader, |item: Self::Item| (
    item.sender.into_string(),
    item.destination.into_string(),
    item.seq_no.as_u64(),
    item.term.as_u64()
));
