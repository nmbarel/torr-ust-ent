use std::{path::Path};
use bencode::{decode::Decoder, Bencode, encode::Encoder};
use sha1::{Digest, Sha1};
use crate::TorrentError;

#[derive(Debug, Clone, PartialEq)]
pub struct TorrentMetadata {
    info_hash: [u8; 20],
    creation_date: Option<i64>,
    announce: String,
    //announce_list: Vec<String>,
    piece_length: i64,
    pieces: Vec<u8>,
    name: String,
    file_structure: FileStructure, // FileStructure::Single(l) means only 1 files called name and length l,
    // FileStructure::Multiple(L) means multiple files called name/L.path and each file with length L.l.
}

#[derive(Debug, Clone, PartialEq)]
enum FileStructure {
    Single(i64),
    Multiple(Vec<TorrentFile>)
}

#[derive(Debug, Clone, PartialEq)]
struct TorrentFile {
    length: i64,
    path: String,
}

impl TorrentMetadata {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, TorrentError> {
        let bytes = std::fs::read(path)
            .map_err(|e| TorrentError::msg(format!("error reading file: {}", e)))?;

        let mut decoder = Decoder::new(bytes);
        let value = decoder
            .decode()
            .map_err(|e| TorrentError::msg(format!("bencode decode error: {}", e)))?;

        Self::from_bencode(value)
    }

    pub fn from_bencode(dict: Bencode) -> Result<Self, TorrentError> {
        // got bencoded dict, need to parse it into TorrentMetadata struct 
        //(hash the info, load all the values from the Bencode::dict dict, choose in the FileStructure enum)

        let torrent_info = Self::get_required(&dict, "info")?;

        let encoder = Encoder::new(torrent_info.clone());
        let info_asbytes = encoder.encode()
            .map_err(|e| TorrentError::msg(format!("failed to re-bencode the info: {}", e)))?;

        let mut hasher = Sha1::new();
        hasher.update(info_asbytes);
        let result = hasher.finalize();

        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&result[..]);

        let announce = Self::get_required(&dict, "announce")?.as_str()
            .ok_or(TorrentError::msg("Cannot translate annouce to str"))?;

        let creation_date = match Self::get_required(&dict, "creation date") {
            Ok(Bencode::Int(time)) => time,
            Ok(_) => &0,
            Err(_) => &0,
        };

        let piece_length = Self::get_required(&torrent_info, "piece length")?
            .as_int().ok_or( TorrentError::msg("Cannot translate piece_length to int!"))?;

        let name = Self::get_required(torrent_info, "name")?
            .as_str().ok_or(TorrentError::msg("Cannot translate name to str"))?;

        let pieces = Self::get_required(&torrent_info, "pieces")?
            .as_bytes().ok_or(TorrentError::msg("Cannot translate pieces to bytes"))?;

        let file_structure = match Self::get_required(&torrent_info, "length") {
            Ok(Bencode::Int(len)) => FileStructure::Single(*len),
            Ok(_) => return Err(TorrentError::msg("length was incorrectly decoded!")),
            Err(_) => {
                let file_list = Self::get_required(&torrent_info, "files")?
                    .as_list().ok_or(TorrentError::msg("Cannot parse files from list"))?;

                let mut file_struct_vec: Vec<TorrentFile> = Vec::new();

                for entry in file_list {
                    let decoded_entry = entry.as_dict()
                        .ok_or(TorrentError::msg("Cannot decode file entry!"))?;

                    let len = decoded_entry.get(&"len".as_bytes().to_vec())
                        .ok_or(TorrentError::msg("Cannot translate len in file entry!"))?
                        .as_int().ok_or(TorrentError::msg("Cannot translate len in file entry!"))?;

                    let path_list = decoded_entry.get(&"path".as_bytes().to_vec())
                        .ok_or(TorrentError::msg("Cannot find path in file entry!"))?
                        .as_list().ok_or(TorrentError::msg("Cannot parse path as list!"))?;

                    let path_strings: Result<Vec<String>, TorrentError> = path_list
                        .iter()
                        .map(|p| {
                            p.as_str()
                                .ok_or(TorrentError::msg("path element is not a string"))
                                .map(|s| s.to_string())
                        })
                        .collect();

                    let path_vec = path_strings?;
                    let path = path_vec.join("/");

                    file_struct_vec.push(TorrentFile { length: len, path: path })
                }

                FileStructure::Multiple(file_struct_vec)
            }
        };

        Ok(TorrentMetadata {
            info_hash,
            creation_date: Some(*creation_date),
            announce,
            piece_length,
            pieces: [].to_vec(),
            name,
            file_structure,

        })
    }

    fn get_required<'a>(dict: &'a Bencode, key: &str) -> Result<&'a Bencode, TorrentError> {
        match dict {
            Bencode::Dict(d) => {
                d.get(&key.as_bytes().to_vec())
                    .ok_or_else(|| TorrentError::msg(format!("Missing '{}' key", key)))
            }
            _ => {
                Err(TorrentError::msg("passed a non-dict Bencode object to get_required!"))
            }
        }
    }
}
