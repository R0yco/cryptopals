use once_cell::sync::Lazy;
use rand::random;
use openssl::symm::{Cipher, encrypt, decrypt};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a = vec![1];
    let enc = encryption_oracle(&a);
    discover_blocksize();
    println!("{}", is_ecb_mode());
    break_ecb();
}

fn compare(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    for (ai, bi) in a.iter().zip(b.iter()) {
        match ai.cmp(&bi) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord
        }
    }

    a.len().cmp(&b.len())
}

fn is_ecb_mode() -> bool{
    let blocksize = discover_blocksize();
    let plaintext = vec![b'A'; 2* blocksize as usize];
    let ciphertext = encryption_oracle(plaintext.as_slice());
    let first_chunk = &ciphertext[0..blocksize as usize];
    let second_chunk = &ciphertext[blocksize as usize.. 2* blocksize as usize];
    if compare(first_chunk, second_chunk) == std::cmp::Ordering::Equal {
        return true
    }
    return false;
}

fn discover_blocksize() -> u8{
    let empty = vec![];
    let initial_len = encryption_oracle(empty.as_slice()).len();
    let mut bytes: String = String::from("A");
    let blocksize;
    loop {
        let curr_len = encryption_oracle(bytes.as_bytes()).len();
        if curr_len != initial_len{
            blocksize = curr_len - initial_len;
            break;
        }
        bytes += "A";
    }
    blocksize as u8
}

fn encryption_oracle(plaintext: &[u8]) -> Vec<u8> {
    let unknown = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let unknown = base64::decode(unknown);
    let mut  plaintext_with_unknown = Vec::new();
    plaintext_with_unknown.extend_from_slice(plaintext);
    plaintext_with_unknown.append(&mut unknown.unwrap());
    let cipher = Cipher::aes_128_ecb();
    //let encryption_key: [u8;16] ENCRYPTION_KEY.
    let ciphertext = encrypt(cipher, &ENCRYPTION_KEY,Some(&vec![1]), &plaintext_with_unknown);
    ciphertext.unwrap()
}

fn make_attack_map(base_plaintext: &[u8]) -> HashMap<String, Vec<u8>>
{
    let blocksize = discover_blocksize();
    let mut attack_map: HashMap<String, Vec<u8>> = HashMap::new();
    for c in 1..255{

        let mut plaintext = Vec::<u8>::new();
        plaintext.extend_from_slice(base_plaintext);
        plaintext.push(c);
        let ciphertext = encryption_oracle(plaintext.as_slice());
        let mut first_block = Vec::new();

        first_block.extend_from_slice(&ciphertext[0..blocksize as usize]);
        let first_block = base64::encode(first_block);

        attack_map.insert(first_block, plaintext);
    }
    attack_map


}

fn break_ecb(){

    let secret_len = encryption_oracle(&[]).len();
    let mut secret = Vec::<u8>::new();
    let mut block_index = 0;
    for i in 0..140{
        let plaintext_len = 15 - secret.len() % 16;
        if  i  == 16 {
            block_index += 1;
            continue
        }
        let mut plaintext = vec![b'A'; plaintext_len];
        let mut secret_plaintext = Vec::new();
        let ciphertext = encryption_oracle(&plaintext);
        
        let to_map: &[u8];
        if secret.len() >= 16{
            to_map = &secret[secret.len()-15 .. secret.len()];
        }
        else
        {
            secret_plaintext.extend_from_slice(&secret[block_index*16..]);
            plaintext.append(&mut secret_plaintext);
            to_map = &plaintext[..];
        }
      
        
        let attack_map = make_attack_map(&to_map);
        //println!("plaintext after adding secret is {:?}",plaintext);
        
        let nth_block = &ciphertext[block_index*16..block_index*16+16];

        let encoded_nth_block = base64::encode(nth_block);
        let discovered_byte = attack_map.get_key_value(&encoded_nth_block).unwrap().1.last().unwrap();
        //println!("{:?}", attack_map.get_key_value(&encoded_nth_block).unwrap());
        secret.push(*discovered_byte);
        println!("{:?}", secret.len());
        if  i  %16 == 0 && i!=0 {
            block_index += 1;
            continue
        }
    }
    println!("{}",String::from_utf8(secret).unwrap());
    



}

static ENCRYPTION_KEY: Lazy<Vec<u8>> = Lazy::new(||{
    let enc_key: [u8;16] = random();
    let mut  key = Vec::new();
    key.extend_from_slice(&enc_key);
    key
});
