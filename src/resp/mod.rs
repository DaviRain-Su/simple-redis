use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;

pub mod decode;
pub mod encode;

pub trait RespEncoder {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecoder {
    fn decode(data: Self) -> Result<RespFrame, String>;
}

pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(Vec<RespFrame>),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    // BigNumber(i64), RUST DOES NOT HAVE BIGINT
    Map(RespMap),
    Set(RespSet),
}

pub struct SimpleString(String);

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

pub struct SimpleError(String);

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BulkString(Vec<u8>);

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RespNull;

pub struct RespArray(Vec<RespFrame>);

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RespNullArray;

pub struct RespNullBulkString;

pub struct RespMap(HashMap<String, RespFrame>);

impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RespSet(HashSet<RespFrame>);

impl Deref for RespSet {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
