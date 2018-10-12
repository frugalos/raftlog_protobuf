//! Encoders and decoders for the constituents defined in `raftlog::state` module.
use bytecodec::ErrorKind;
use protobuf_codec::field::num::{F1, F2, F3};
use protobuf_codec::field::{FieldDecoder, FieldEncoder, Fields, MaybeDefault, Repeated};
use protobuf_codec::message::{MessageDecoder, MessageEncoder};
use protobuf_codec::scalar::{
    StringDecoder, StringEncoder, Uint32Decoder, Uint32Encoder, Uint64Decoder, Uint64Encoder,
};
use raftlog::cluster::{ClusterConfig, ClusterState};
use raftlog::election::Ballot;
use raftlog::node::NodeId;

/// Decoder for `Ballot`.
#[derive(Debug, Default)]
pub struct BallotDecoder {
    inner: MessageDecoder<
        Fields<(
            MaybeDefault<FieldDecoder<F1, Uint64Decoder>>,
            MaybeDefault<FieldDecoder<F2, StringDecoder>>,
        )>,
    >,
}
impl_message_decode!(BallotDecoder, Ballot, |t: (u64, String)| Ok(Ballot {
    term: t.0.into(),
    voted_for: t.1.into()
}));

/// Encoder for `Ballot`.
#[derive(Debug, Default)]
pub struct BallotEncoder {
    inner: MessageEncoder<
        Fields<(
            FieldEncoder<F1, Uint64Encoder>,
            FieldEncoder<F2, StringEncoder>,
        )>,
    >,
}
impl_sized_message_encode!(BallotEncoder, Ballot, |item: Self::Item| (
    item.term.as_u64(),
    item.voted_for.into_string()
));

/// Decoder for `ClusterConfig`.
#[derive(Debug, Default)]
pub struct ClusterConfigDecoder {
    inner: MessageDecoder<
        Fields<(
            Repeated<FieldDecoder<F1, StringDecoder>, Vec<String>>,
            Repeated<FieldDecoder<F2, StringDecoder>, Vec<String>>,
            MaybeDefault<FieldDecoder<F3, Uint32Decoder>>,
        )>,
    >,
}
impl_message_decode!(
    ClusterConfigDecoder,
    ClusterConfig,
    |t: (Vec<_>, Vec<_>, u32)| Ok(ClusterConfig::with_state(
        t.0.into_iter().map(NodeId::from).collect(),
        t.1.into_iter().map(NodeId::from).collect(),
        match t.2 {
            0 => ClusterState::Stable,
            1 => ClusterState::CatchUp,
            2 => ClusterState::Joint,
            n => track_panic!(ErrorKind::InvalidInput, "Unknown cluster state: {}", n),
        }
    ))
);

/// Encoder for `ClusterConfig`.
#[derive(Debug, Default)]
pub struct ClusterConfigEncoder {
    inner: MessageEncoder<
        Fields<(
            Repeated<FieldEncoder<F1, StringEncoder>, Vec<String>>,
            Repeated<FieldEncoder<F2, StringEncoder>, Vec<String>>,
            MaybeDefault<FieldEncoder<F3, Uint32Encoder>>,
        )>,
    >,
}
impl_message_encode!(ClusterConfigEncoder, ClusterConfig, |item: Self::Item| (
    item.new_members()
        .iter()
        .map(|n| n.clone().into_string())
        .collect(),
    item.old_members()
        .iter()
        .map(|n| n.clone().into_string())
        .collect(),
    match item.state() {
        ClusterState::Stable => 0,
        ClusterState::CatchUp => 1,
        ClusterState::Joint => 2,
    }
));
