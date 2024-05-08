use super::{
    BulkString, RespArray, RespFrame, RespMap, RespNullBulkString, RespSet, SimpleError,
    SimpleString,
};
use crate::resp::RespEncoder;

const BUFF_CAP: usize = 4096;

impl RespEncoder for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(s) => s.encode(),
            RespFrame::Error(e) => e.encode(),
            RespFrame::Integer(i) => i.encode(),
            RespFrame::BulkString(b) => b.encode(),
            RespFrame::NullBulkString(n) => n.encode(),
            // RespFrame::Array(a) => a.encode(),
            // RespFrame::Null(n) => n.encode(),
            // RespFrame::NullArray(n) => n.encode(),
            // RespFrame::Boolean(b) => b.encode(),
            // RespFrame::Double(d) => d.encode(),
            // RespFrame::Map(m) => m.encode(),
            // RespFrame::Set(s) => s.encode(),
            _ => todo!(),
        }
    }
}

impl RespEncoder for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespEncoder for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespEncoder for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespEncoder for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.len()).as_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespEncoder for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

impl RespEncoder for RespArray {
    fn encode(self) -> Vec<u8> {
        // let capacity = self.iter().map(|frame| frame.encode().len()).sum::<usize>() + 16;
        let mut buf = Vec::with_capacity(BUFF_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.0.len()).as_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespEncoder for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

impl RespEncoder for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "-" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };
        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

impl RespEncoder for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUFF_CAP);
        buf.extend_from_slice(format!("%{}\r\n", self.len()).as_bytes());
        for (k, v) in self.0 {
            buf.extend_from_slice(&SimpleString::new(k).encode());
            buf.extend_from_slice(&v.encode());
        }
        buf
    }
}

impl RespEncoder for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUFF_CAP);
        buf.extend_from_slice(format!("~{}\r\n", self.len()).as_bytes());
        for v in self.0 {
            buf.extend_from_slice(&v.encode());
        }
        buf
    }
}
