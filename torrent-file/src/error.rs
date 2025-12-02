#[derive(Debug)]
pub enum TorrentError {
    Message(String),
}

impl std::fmt::Display for TorrentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorrentError::Message(msg) => 
                write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TorrentError {}
