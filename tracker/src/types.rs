use std::net::Ipv4Addr;

#[derive(Debug)]

pub struct TrackerRequest {
    announce_url: String,
    info_hash: [u8; 20],
    peer_id: [u8; 20],
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    compact: bool,
}

#[derive(Debug)]
pub struct TrackerResponse {
    interval: u32,
    peers: Vec<PeerInfo>,
}

#[derive(Debug)]
pub struct PeerInfo {
    ip: Ipv4Addr,
    port: u16,
    peer_id: Option<[u8; 20]>,
}

impl TrackerRequest {
    pub fn new(announce_url: impl Into<String>, info_hash: [u8; 20], peer_id: [u8; 20],
               port: u16, uploaded: u64, downloaded: u64, left: u64, compact: bool) -> Self {
                Self {
                    announce_url: announce_url.into(),
                    info_hash,
                    peer_id,
                    port,
                    uploaded,
                    downloaded,
                    left,
                    compact,
                }
    }

    pub fn announce_url(&self) -> &str { &self.announce_url }
    pub fn info_hash(&self) -> &[u8; 20] { &self.info_hash }
    pub fn peer_id(&self) -> &[u8; 20] { &self.peer_id }
    pub fn port(&self) -> u16 { self.port }
    pub fn uploaded(&self) -> u64 { self.uploaded }
    pub fn downloaded(&self) -> u64 { self.downloaded }
    pub fn left(&self) -> u64 { self.left }
    pub fn compact(&self) -> bool { self.compact }
}

impl TrackerResponse {
    pub fn new(interval: u32, peers: Vec<PeerInfo>) -> Self {
        Self{ interval, peers }
    }
}

impl PeerInfo {
    pub fn new(ip: Ipv4Addr, port: u16, peer_id: Option<[u8; 20]>) -> Self {
        Self { ip, port, peer_id }
    }
}