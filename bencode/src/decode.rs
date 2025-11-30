use crate::{Bencode, BencodeError};

pub struct Decoder {
    input: Vec<u8>,
    pos: usize,
}

impl Decoder {
    pub fn new(input: Vec<u8>) -> Self {
        Self { input, pos: 0 }
    }

    pub fn decode(&mut self) -> Result<Bencode, BencodeError> {
        let value = self.parse_value()?;

        if self.pos != self.input.len() {
            return Err(BencodeError::Message("Too many bencode values, use decode_all".to_string()));
        }

        Ok(value)
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let b = self.input.get(self.pos).copied();
        if b.is_some() {
            self.pos += 1;
        }

        b
    }

    fn parse_value(&mut self) -> Result<Bencode, BencodeError> {
        match self.peek() {
            Some(b'i') => self.parse_int(),
            Some(b'l') => self.parse_list(),
            Some(b'd') => self.parse_dict(),
            Some(b'0'..=b'9') => self.parse_bytes(),
            Some(_) => Err(BencodeError::InvalidFormat),
            None => Err(BencodeError::UnexpectedEOF),
        }
    }

    fn parse_int(&mut self) -> Result<Bencode, BencodeError> {
        let b = self.next().ok_or(BencodeError::UnexpectedEOF)?;
        if b != b'i' { 
            return Err(BencodeError::InvalidFormat);
        }

        let mut buf = Vec::new();
        while let Some(&b) = self.input.get(self.pos) {
            if b == b'e' {break};
            buf.push(b);
            self.pos += 1;
        }
        if self.next() != Some(b'e') {
            return Err(BencodeError::InvalidFormat);
        }

        let num_str = std::str::from_utf8(&buf).map_err(|_| BencodeError::InvalidInteger)?;
        let n: i64 = num_str.parse().map_err(|_| BencodeError::InvalidInteger)?;

        Ok(Bencode::Int(n))
    }

    fn parse_bytes(&mut self) -> Result<Bencode, BencodeError> {
        let b = self.next().ok_or(BencodeError::UnexpectedEOF)?;
        if !(b'0'..=b'9').contains(&b) { 
            return Err(BencodeError::InvalidFormat);
        }
        let mut len = Vec::new();
        len.push(b);
        while let Some(&b) = self.input.get(self.pos) {
            if b == b':' {break};
            len.push(b);
            self.pos += 1;
        }
        if self.next() != Some(b':') {
            return Err(BencodeError::InvalidFormat);
        }
        let to_read: usize = std::str::from_utf8(&len).map_err(|_| BencodeError::Message("from utf-8 err".to_string()))?.parse().map_err(|_| BencodeError::Message("parse error".to_string()))?;

        if self.pos + to_read > self.input.len() {
            return Err(BencodeError::UnexpectedEOF);
        }

        let data = self.input[self.pos..self.pos+to_read].to_vec();
        self.pos += to_read;

        Ok(Bencode::Bytes(data))

    }

    fn parse_list(&mut self) -> Result<Bencode, BencodeError> {
        if self.next() != Some(b'l') {
            return Err(BencodeError::InvalidFormat);
        }

        let mut items = Vec::new();
        loop {
            match self.peek() {
                Some(b'e') => {
                    self.next();
                    break;
                }
                Some(_) => {
                    items.push(self.parse_value()?);
                }
                None => return Err(BencodeError::UnexpectedEOF),
            }
        }
        Ok(Bencode::List(items))
    }

    fn parse_dict(&mut self) -> Result<Bencode, BencodeError> {
        if self.next() != Some(b'd') {
            return Err(BencodeError::InvalidFormat);
        }

        let mut map = std::collections::BTreeMap::new();
        loop {
            match self.peek() {
                Some(b'e') => {
                    self.next();
                    break;
                }
                Some(_) => {
                    let key = match self.parse_bytes()? {
                        Bencode::Bytes(b) => b,
                        _ => return Err(BencodeError::InvalidFormat),
                    };
                    let value = self.parse_value()?;
                    map.insert(key, value);
                }
                None => return Err(BencodeError::UnexpectedEOF),
            }
        }
        Ok(Bencode::Dict(map))
    }
}