use std::net::Ipv4Addr;

use reqwest::Client;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use crate::{PeerInfo, TrackerRequest, TrackerResponse, TrackerError};
use bencode::{Bencode, decode::Decoder};
use torrent_file::{TorrentError, get_required};

pub async fn announce(client: &Client, req: &TrackerRequest) -> Result<TrackerResponse, Box<dyn std::error::Error>> {
    let info_hash_enc = percent_encode(req.info_hash(), NON_ALPHANUMERIC);
    let peer_id_enc = percent_encode(req.peer_id(), NON_ALPHANUMERIC);

    let url = format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}",
        req.announce_url(),
        info_hash_enc,
        peer_id_enc,
        req.port(),
        req.uploaded(),
        req.downloaded(),
        req.left(),
        if req.compact() { 1 } else { 0 }
    );

    let bytes = client.get(url).send().await?.bytes().await?;

    let mut decoder = Decoder::new(bytes.to_vec());
    let b = decoder.decode()?;

    let interval = get_required(&b, "interval")?.as_int().unwrap_or(1800) as u32;
    let peers_val = get_required(&b, "peers")?;

    let peers = if let Some(bytes) = peers_val.as_bytes() {
        //Compact Peers
        parse_compact_peers(bytes)?
    } else if let Some(list) = peers_val.as_list() {
        //Old peers
        parse_dict_peers(list)?
    } else {
        return Err("Invalid peers format".into());
    };
    
    let temp = TrackerResponse::new(interval, peers);

    Ok(temp)


}

fn parse_compact_peers(bytes: &[u8]) -> Result<Vec<PeerInfo>, TrackerError> {
    if bytes.len() % 6 != 0 {
        return Err(TrackerError::InvalidPeerLength);
    }

    let mut peers = Vec::new();
    for peer_bytes in bytes.chunks(6) {
        let ip = Ipv4Addr::new(peer_bytes[0], peer_bytes[1], peer_bytes[2], peer_bytes[3]);
        let port = u16::from_be_bytes([peer_bytes[4], peer_bytes[5]]);

        peers.push(PeerInfo::new(ip, port, None));
    }

    Ok(peers)
}

fn parse_dict_peers(list: &[Bencode]) -> Result<Vec<PeerInfo>, Box<dyn std::error::Error>> {
    let mut peers = Vec::new();

    for peer in list {
        let ip: Ipv4Addr = get_required(&peer, "ip")?.as_str().ok_or(TorrentError::msg("could not convert ip to str"))?.parse()?;
        let port = get_required(&peer, "port")?.as_int().ok_or(TorrentError::msg("could not convert port to int"))? as u16;
        let peer_id_bytes = get_required(&peer, "peer_id")?.as_bytes().ok_or(TorrentError::msg("could not convert peer_id to str"))?;
        let peer_id: [u8; 20] = peer_id_bytes.try_into().map_err(|_| TorrentError::msg("peer_id must be exactly 20 bytes"))?;

        peers.push(PeerInfo::new(ip, port, Some(peer_id)))
    }

    Ok(peers)
}