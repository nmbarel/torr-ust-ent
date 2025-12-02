use crate::{Bencode, BencodeError};

pub struct Encoder {
    input: Bencode,
}

impl Encoder {
    pub fn new(input: Bencode) -> Self {
        Self { input }
    }

    pub fn encode(&self) -> Result<Vec<u8>, BencodeError> {
        Self::encode_value(&self.input)
    }

    fn encode_value(value: &Bencode) -> Result<Vec<u8>, BencodeError> {
        match value {
            Bencode::Int(i) => {
                let mut result = Vec::new();
                result.push(b'i');
                result.extend_from_slice(i.to_string().as_bytes());
                result.push(b'e');
                Ok(result)
            }
            Bencode::Bytes(b) => {
                let mut result = Vec::new();
                result.extend_from_slice(b.len().to_string().as_bytes());
                result.push(b':');
                result.extend_from_slice(b.as_slice());
                Ok(result)
            }
            Bencode::List(l) => {
                let mut result = Vec::new();
                result.push(b'l');
                for item in l.iter() {
                    let enc = Self::encode_value(item)?;
                    result.extend_from_slice(&enc);
                }
                result.push(b'e');
                Ok(result)
            }
            Bencode::Dict(d) => {
                let mut result = Vec::new();
                result.push(b'd');
                for (k, v) in d.iter() {
                    result.extend_from_slice(k.len().to_string().as_bytes());
                    result.push(b':');
                    result.extend_from_slice(k.as_slice());
                    let enc = Self::encode_value(v)?;
                    result.extend_from_slice(&enc);
                }
                result.push(b'e');
                Ok(result)
            }
        }
    }
}