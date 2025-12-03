use std::borrow::Cow;

#[derive(Debug)]
pub enum TorrentError {
    Message(Cow<'static, str>),
}

impl TorrentError {
    pub fn msg<S: Into<Cow<'static, str>>>(s: S) -> Self {
        TorrentError::Message(s.into())
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for TorrentError {
    fn from(s: S) -> Self {
        TorrentError::Message(s.into())
    }
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
