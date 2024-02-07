use serde::{Serialize, Deserialize};
use serde_json::{from_slice as from_json, to_vec as to_json};
use crate::errors::MessageError;

pub struct Encoder;

impl Encoder
{
    pub fn encode<T>(s: T) -> Result<Vec<u8>, MessageError>
        where T: Serialize
    {
        to_json(&s).map_err(|err| MessageError::CannotSerialize(err.to_string()))
    }
}

pub struct Decoder;

impl Decoder
{
    pub fn decode<'a, T>(b: &'a [u8]) -> Result<T, MessageError>
        where T: Deserialize<'a>
    {
        from_json(b).map_err(|err| MessageError::CannotDeserialize(err.to_string()))
    }
}