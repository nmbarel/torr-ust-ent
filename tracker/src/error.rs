use std::borrow::Cow;

#[derive(Debug)]
pub enum TrackerError {
    InvalidPeerLength,
    InvalidDictPeerFormat,
    Message(Cow<'static, str>),
}

impl TrackerError {
    pub fn msg<S: Into<Cow<'static, str>>>(s: S) -> Self {
        TrackerError::Message(s.into())
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for TrackerError {
    fn from(s: S) -> Self {
        TrackerError::Message(s.into())
    }
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackerError::Message(msg) => 
                write!(f, "{}", msg),
            TrackerError::InvalidPeerLength =>
                write!(f, "Invalid peer length!"),
            TrackerError::InvalidDictPeerFormat =>
                write!(f, "Invalid Dict Peer Format!")
        }
    }
}

impl std::error::Error for TrackerError {}