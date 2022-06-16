use helpers::cbc::cbc_decrypt;
use helpers::padding::strip_padding;
use once_cell::sync::Lazy;
use openssl::symm::{decrypt, encrypt, Cipher};
use pkcs7::pad;
use rand::seq::SliceRandom;
use std::convert::TryInto;

fn main() {
    //println!("{}", decrypt_and_check_padding(&oracle()));
    find_plaintext_byte_in_last_block();
}

fn find_plaintext_byte_in_last_block() {
    let mut ciphertext = oracle();
    //let mut block = &mut ciphertext[ciphertext.len()-16..ciphertext.len()];
    let ciphertext_len = ciphertext.len();
    let mut zeroing_iv: [u8; 16] = [0; 16];
    for idx in (0..16).rev() {
        for val in 0..255 {
            ciphertext[ciphertext_len - 32+idx] = val;
            //println!("{:?}",ciphertext);
            if decrypt_and_check_padding(&ciphertext) {
                if idx == 15 {
                    let mut validation_ciphertext = ciphertext.clone();
                    validation_ciphertext[ciphertext_len-17] ^= 0x1;
                    if !decrypt_and_check_padding(&validation_ciphertext){
                        continue;
                    }
                }
                zeroing_iv[idx-1] = val ^ 1;
                print!("{}", val);
            }
        }
    }
}

fn get_random_string() -> Vec<u8> {
    STRINGS.choose(&mut rand::thread_rng()).unwrap().to_vec()
}

fn oracle() -> Vec<u8> {
    let mut random_string = get_random_string();
    pad(&mut random_string, 16);
    let cipher = Cipher::aes_128_cbc();
    encrypt(cipher, &ENCRYPTION_KEY, Some(&[0; 16]), &random_string).unwrap()
}

fn decrypt_and_check_padding(ciphertext: &[u8]) -> bool {
    let cipher = Cipher::aes_128_cbc();
    let plaintext = match decrypt(cipher, &ENCRYPTION_KEY, Some(&[0; 16]), ciphertext) {
        Ok(decrypted) => decrypted,
        Err(_) => vec![1],
    };
    //let  plaintext = cbc_decrypt(ciphertext, &ENCRYPTION_KEY);
    match strip_padding(&plaintext) {
        Ok(_) => true,
        Err(_) => false,
    }
}

static STRINGS: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    vec![
        "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc="
            .as_bytes()
            .to_vec(),
        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic="
            .as_bytes()
            .to_vec(),
        "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw=="
            .as_bytes()
            .to_vec(),
        "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg=="
            .as_bytes()
            .to_vec(),
        "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl"
            .as_bytes()
            .to_vec(),
        "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA=="
            .as_bytes()
            .to_vec(),
        "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw=="
            .as_bytes()
            .to_vec(),
        "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8="
            .as_bytes()
            .to_vec(),
        "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g="
            .as_bytes()
            .to_vec(),
        "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
            .as_bytes()
            .to_vec(),
    ]
});

static ENCRYPTION_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    let enc_key: [u8; 16] = rand::random();
    let mut key = Vec::new();
    key.extend_from_slice(&enc_key);
    key
});
