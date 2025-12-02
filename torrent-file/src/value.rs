use std::path::Path;
use bencode::{decode::Decoder, Bencode};
use crate::TorrentError;

#[derive(Debug, Clone, PartialEq)]
pub struct TorrentMetadata {
    info_hash: [u8; 20],
    announce: Option<String>,
    announce_list: Vec<String>,
    piece_length: u64,
    pieces: Vec<u8>,
    name: String,
    file_structure: FileStructure, // FileStructure::Single(l) means only 1 files called name and length l,
    // FileStructure::Multiple(L) means multiple files called name/L.path and each file with length L.l.
}

#[derive(Debug, Clone, PartialEq)]
enum FileStructure {
    Single(u64),
    Multiple(Vec<TorrentFile>)
}

#[derive(Debug, Clone, PartialEq)]
struct TorrentFile {
    length: u64,
    path: Vec<String>,
}

impl TorrentMetadata {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, TorrentError> {
        let bytes = std::fs::read(path)
            .map_err(|e| TorrentError::Message(format!("error reading file: {}", e)))?;

        let mut decoder = Decoder::new(bytes);
        let value = decoder
            .decode()
            .map_err(|e| TorrentError::Message(format!("bencode decode error: {}", e)))?;

        Self::from_bencode(value)
    }

    pub fn from_bencode(dict: Bencode) -> Result<Self, TorrentError> {
        todo!() // got bencoded dict, need to parse it into TorrentMetadata struct (hash the info, load all the values from the Bencode::dict dict, choose in the FileStructure enum)
    }
}
