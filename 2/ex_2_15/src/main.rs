use std::fmt;
use std::error::Error;
use once_cell::sync::Lazy;
use rand::random;
use openssl::symm::{Cipher, encrypt, decrypt};

type Result<T> = std::result::Result<T, PaddingError>;

#[derive(Debug, Clone)]
struct PaddingError
{
    details: String,
}

impl fmt::Display for PaddingError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.details)
    }
}
impl Error for PaddingError {
    fn description(&self) -> &str {
        &self.details
    }
}
impl PaddingError {
    fn new(msg: &str) -> PaddingError {
        PaddingError{details: msg.to_string()}
    }
}
fn main() {
    println!("Hello, world!");
    let correctly_padded = "ICE ICE BABY\x05\x05\x05\x05";
    println!("{:?}",";admin=true;".as_bytes());
    //let stripped = strip_padding(correctly_padded.as_bytes()).unwrap();
    //println!("{:?}", stripped);
    
    let mut ciphertext = oracle(&['a' as u8,'b' as u8,'c' as u8]);
    ciphertext[0] = ciphertext[0] ^ 'b' as u8  ^ 'a' as u8 ;
    println!("ciphertext[16] is {} and ciphertext[0] is {}",ciphertext[16], ciphertext[0]);
    println!("{:?}", ciphertext);
    decrypt_and_find_admin(&ciphertext);
}

fn strip_padding(text: &[u8]) -> Result<Vec<u8>>
{
    if text.len() != 16{
        return Err(PaddingError::new("invalid length for padding"));
    }
    let last_byte = text[15];
    if !(0..16).contains(&last_byte)
    {
        return Err(PaddingError::new("not properly padded"));
    }
   for i in (16-last_byte)..16
   {
        if text[i as usize] != last_byte
        {
            return Err(PaddingError::new("not properly padded"));
        }
   }
   Ok((&text[0 as usize..16-last_byte as usize]).to_vec())
}

fn oracle(plaintext: &[u8]) -> Vec<u8>
{
    let prefix = "comment1=cooking%20MCs;userdata=".as_bytes();
    let suffix = ";comment2=%20like%20a%20pound%20of%20bacon".as_bytes();
    let filtered_plaintext = plaintext.to_vec();
    let mut filtered_plaintext: Vec<u8> = filtered_plaintext.into_iter().filter(|e| *e != '=' as u8 && *e != ';' as u8).collect(); 
    let mut final_plaintext = Vec::new();
    final_plaintext.extend_from_slice(prefix);
    final_plaintext.append(&mut filtered_plaintext);
    final_plaintext.extend_from_slice(suffix);

    let cipher = Cipher::aes_128_cbc();
    let mut ciphertext = encrypt(cipher, &ENCRYPTION_KEY, Some(&[0;16]), &final_plaintext);
    ciphertext.unwrap()
    //println!("{:?}", String::from_utf8(final_plaintext).unwrap());
    
}

fn decrypt_and_find_admin(ciphertext: &[u8]) -> bool
{
    let cipher = Cipher::aes_128_cbc();
    let plaintext = decrypt(cipher,&ENCRYPTION_KEY,Some(&[0;16]),ciphertext).unwrap();
    // for block in 0..plaintext.len()% 16 -1{

    //     let plaintext_chunk = match String::from_utf8(plaintext[block*16..block*16+16].to_vec()){
    //         Ok(a) => a,
    //         Err(_) => String::from("none"),

    //     };
        
    //     println!("{}", plaintext_chunk);
    // }
    println!("{:?}",plaintext);
    //let plaintext = String::from_utf8(plaintext).unwrap();
    //plaintext.contains(";admin=true;")
    true
}

static ENCRYPTION_KEY: Lazy<Vec<u8>> = Lazy::new(||{
    let enc_key: [u8;16] = random();
    let mut  key = Vec::new();
    key.extend_from_slice(&enc_key);
    key
});
