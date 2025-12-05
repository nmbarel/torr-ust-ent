use bencode::Bencode;
use bencode::decode::Decoder;
use bencode::encode::Encoder;
use torrent_file::value::{FileStructure, TorrentMetadata};
use reqwest::Client;
use tracker::TrackerRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // A few example bencoded values
    // let examples = [
    //     b"4:spam".to_vec(),                    // bytes -> "spam"
    //     b"i42e".to_vec(),                      // int -> 42
    //     b"l4:spam4:eggse".to_vec(),            // list -> ["spam","eggs"]
    //     b"d3:cow3:moo4:spam4:eggse".to_vec(),  // dict -> {"cow":"moo","spam":"eggs"}
    // ];

    // for input in &examples {
    //     let mut dec = Decoder::new(input.clone());
    //     match dec.decode() {
    //         Ok(value) => println!("decoded: {:#?}", value),
    //         Err(e) => eprintln!("decode error: {:?}", e),
    //     }
    // }

    // Single example
    let mut dec = Decoder::new(b"d8:announce53:http://files.publicdomaintorrents.com/bt/announce.php13:creation datei1134755461e4:infod6:lengthi317625051e4:name18:A_Star_is_Born.mp412:piece lengthi262144eee".to_vec());
    // match dec.decode() {
    //     Ok(value) => println!("single decode: {:#?}", value.),
    //     Err(e) => eprintln!("error: {:?}", e),
    // }

    let temp = dec.decode().unwrap();
    let value = temp.as_dict().unwrap();
    for key in value.keys() {
        println!("{:?}", str::from_utf8(key).unwrap())
    }
    for value in value.values() {
        match value {
            Bencode::Bytes(s) => println!("{:?}", str::from_utf8(s).unwrap()),
            Bencode::Int(i) => println!("{:?}", i),
            Bencode::List(l) => println!("list!"),
            Bencode::Dict(d) => println!("dict!")
        }
    }

    //Encoder
    let input = b"d3:cow3:moo4:spam4:eggse".to_vec();
    let mut dec = Decoder::new(input.clone());
    let value = dec.decode().expect("failed to decode dict");
    let enc = Encoder::new(value);
    match enc.encode() {
        Ok(value) => println!("encoding: {:#?}", String::from_utf8_lossy(&value).to_string()),
        Err(e) => eprintln!("error: {:?}", e),
    }

    //File decode
    let parsedTorrent = TorrentMetadata::from_file("C:\\Users\\User\\Downloads\\AnduinOS-1.4.1-zh_TW.iso.torrent");

    match parsedTorrent {
        Ok(tm) => {
            //println!("{:#?}", tm);

            let client = Client::new();

            let peer_id: [u8; 20] = [0; 20];
            let req = TrackerRequest::new(
                tm.announce().clone(),
                *tm.info_hash(),
                peer_id,
                6881,
                0,
                0,
                317625051, // need to make a match where if single just take the len and if multiple add all of the lens
                true,
            );

            println!("{:#?}", req);

            let resp = tracker::http::announce(&client, &req).await?;
            println!("{:#?}", resp)
        },
        Err(e) => eprintln!("{}", e)
    }

    Ok(())

}