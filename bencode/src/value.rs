use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Bencode {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<Bencode>),
    Dict(BTreeMap<Vec<u8>, Bencode>),
}

impl Bencode {
    pub fn as_int(&self) -> Option<i64> {
        if let Bencode::Int(i) = self { Some(*i) } else { None }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        if let Bencode::Bytes(b) = self { Some(b) } else { None }
    }

    pub fn as_str(&self) -> Option<String> {
        if let Bencode::Bytes(s) = self {Some(String::from_utf8_lossy(s).to_string())} else { None }
    }

    pub fn as_list(&self) -> Option<&[Bencode]> {
        if let Bencode::List(l) = self { Some(l) } else { None }
    }

    pub fn as_dict(&self) -> Option<&BTreeMap<Vec<u8>, Bencode>> {
        if let Bencode::Dict(d) = self { Some(d) } else { None }
    }
}