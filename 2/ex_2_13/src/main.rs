use url::Url;
use once_cell::sync::Lazy;
use rand::random;
use openssl::symm::{Cipher, encrypt, decrypt};

fn main() {
   let profile_string = profile_for("aaaaaaaaaaadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b@example.com");
   println!("{}",profile_string.len());
   println!("{}",profile_string);
   let profile = parse_arguments(&profile_string);
   let ciphertext = profile.get_encrypted_profile();
   let mut admin_block = ciphertext[16..32].to_vec();
   let profile_string2 = profile_for("ex@example.com");
   println!("{} {}",profile_string2, profile_string2.len()-4);
   let profile2 = parse_arguments(&profile_string2);
   let ciphertext2 = profile2.get_encrypted_profile();
   let mut new_ciphertext = ciphertext2[0..32].to_vec();
   
   new_ciphertext.append(&mut admin_block);
   println!("{:?}", new_ciphertext);
   let profile2 = decrypt_and_parse(&new_ciphertext);
   println!("{}", ciphertext2.len());
   println!("{}", new_ciphertext.len());
   println!("{:?}",profile2);

}

fn decrypt_and_parse(ciphertext: &[u8]) ->Profile
{
    let cipher = Cipher::aes_128_ecb();
    let plaintext = decrypt(cipher, &ENCRYPTION_KEY, Some(&vec![1]), ciphertext).unwrap();
    let plaintext = String::from_utf8(plaintext).unwrap();
    parse_arguments(&plaintext)
}
fn parse_arguments(args_string: &str) -> Profile
{
    let mut dummy_url = String::from("http://example.com?");
    dummy_url.push_str(&args_string);
    let dummy_url = Url::parse(&dummy_url).unwrap();
    let mut pairs = dummy_url.query_pairs();
    let email = pairs.next().unwrap().1.to_mut().clone();
    let uid = pairs.next().unwrap().1.to_mut().clone().parse::<u64>().unwrap();
    let role = pairs.next().unwrap().1.to_mut().clone();
    
    
    Profile{ email, uid, role}
}

fn profile_for(email: &str) -> String
{
    let email_sanitized = String::from(email.replace("&", "").replace("=", ""));
    let profile = Profile{email: email_sanitized, uid: 1, role: String::from("user")};
    profile.get_encoded_string()
}

#[derive(Debug)]
struct Profile{
    email: String,
    uid: u64,
    role: String,
}

impl Profile{
    fn get_encoded_string(self) -> String{
        format!("email={}&uid={}&role={}",self.email,self.uid, self.role)
    }
    fn get_encrypted_profile(self) ->Vec<u8>{
        let cipher = Cipher::aes_128_ecb();
        let ciphertext = encrypt(cipher, &ENCRYPTION_KEY,Some(&vec![1]), self.get_encoded_string().as_bytes());
        ciphertext.unwrap()
    }
}


static ENCRYPTION_KEY: Lazy<Vec<u8>> = Lazy::new(||{
    let enc_key: [u8;16] = random();
    let mut  key = Vec::new();
    key.extend_from_slice(&enc_key);
    key
});
