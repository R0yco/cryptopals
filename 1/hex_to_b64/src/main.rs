extern crate base64;
extern crate hex;
fn main() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let b64 = hex_to_b64(hex_str);
    println!("{}",b64);
}


fn hex_to_b64(hex_str: &str) -> String{
    let bytes = hex::decode(hex_str).expect("invalid hex string");
    let b64 = base64::encode(bytes);
    b64
}

