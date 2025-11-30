
use bencode::decode::Decoder;
use bencode::encode::Encoder;
use bencode::Bencode;

fn main() {
    // A few example bencoded values
    let examples = [
        b"4:spam".to_vec(),                    // bytes -> "spam"
        b"i42e".to_vec(),                      // int -> 42
        b"l4:spam4:eggse".to_vec(),            // list -> ["spam","eggs"]
        b"d3:cow3:moo4:spam4:eggse".to_vec(),  // dict -> {"cow":"moo","spam":"eggs"}
    ];

    for input in &examples {
        let mut dec = Decoder::new(input.clone());
        match dec.decode() {
            Ok(value) => println!("decoded: {:#?}", value),
            Err(e) => eprintln!("decode error: {:?}", e),
        }
    }

    // Single example
    let mut dec = Decoder::new(b"4:spam".to_vec());
    match dec.decode() {
        Ok(value) => println!("single decode: {:#?}", value.as_str().unwrap()),
        Err(e) => eprintln!("error: {:?}", e),
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
}