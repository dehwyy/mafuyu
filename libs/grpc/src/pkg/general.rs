#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsOkResponse {
    #[prost(bool, tag = "1")]
    pub is_ok: bool,
}
