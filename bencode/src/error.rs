#[derive(Debug)]
pub enum BencodeError {
    UnexpectedEOF,
    InvalidInteger,
    InvalidFormat,
    Message(String),
}

impl std::fmt::Display for BencodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BencodeError::UnexpectedEOF => 
                write!(f, "Unexpected end of file"),
            BencodeError::InvalidInteger => 
                write!(f, "Invalid integer"),
            BencodeError::InvalidFormat => 
                write!(f, "invalid bencode format"),
            BencodeError::Message(msg) => 
                write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for BencodeError {}