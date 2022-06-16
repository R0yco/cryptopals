use openssl::symm::{Cipher, decrypt, Crypter,Mode};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

use base64;
use hamming;

fn main() {
    let data = [1,2,3,4,5];
    let padded_block = pkcs7_pad(&data, 16);
    //println!("{:?}", padded_block);
    let cipher = Cipher::aes_128_ecb();
    let key = b"YELLOW SUBMARINE";
    let iv = b"1";
    let contents = fs::read_to_string("C:\\Users\\royco\\learning\\cryptopals\\2\\block-cipher\\7.txt")
    .expect("Something went wrong reading the file");
    let contents = contents.replace("\n","");
    let ciphertext = base64::decode(contents);
    let cleartext = decrypt(cipher,key,Some(iv),ciphertext.unwrap().as_slice()).unwrap();
    let output = std::str::from_utf8(cleartext.as_slice());

    let contents2 =  fs::read_to_string("C:\\Users\\royco\\learning\\cryptopals\\2\\block-cipher\\10.txt")
    .expect("Something went wrong reading the file");
    let contents2 = contents2.replace("\n","");
    let ciphertext = base64::decode(contents2).unwrap();
    let test = base64::encode(&ciphertext[0..16]);
    println!("{}", test);



    cbc_decrypt(&ciphertext);
    
}


fn pkcs7_pad(data: &[u8], needed_blocksize: u64) -> Vec<u8>
{
    let padding_amount = needed_blocksize - data.len() as u64;
    let mut block = data.to_vec();
    let mut padding = vec![padding_amount as u8;padding_amount as usize];
    &block.append(&mut padding);
    block
}

fn detect_ecb_encrypted() -> Vec<u8>
{
    let mut distances_indexes = BTreeMap::new();
    if let Ok(values) = read_lines("C:\\Users\\royco\\learning\\cryptopals\\2\\block-cipher\\8.txt"){
        for (index, line) in values.enumerate(){
            let avg_dist = calculate_avg_dist(line.unwrap().as_bytes());
            distances_indexes.insert(avg_dist, index);
        }
    };
    println!("{}", distances_indexes.iter().next().unwrap().1);
    vec![1]
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_avg_dist(ciphertext: &[u8]) -> u64
{
    let mut sum = 0;
    for i in (0..ciphertext.len()).step_by(16){
        for j in (i..ciphertext.len()).step_by(16){
            if i==j{
                continue;
            }
            let chunk1 =  &ciphertext[i..i+16];
            let chunk2 =  &ciphertext[j..j+16];
            let distance = hamming::distance(chunk1, chunk2);
            sum += distance;
        }
    }
    sum / (ciphertext.len() as u64 / 16 * ((ciphertext.len() as u64 / 16) -1)/2)
    
}

fn cbc_decrypt(ciphertext: &[u8]) -> Vec<u8>
{
    let iv= [b'\0';16];
    let iv2 = b"1";
    println!("{}", ciphertext.len());   
    let mut plaintext = Vec::new();
    let first_block = &ciphertext[0..16];

    println!("{:?}", first_block);
    let cipher = Cipher::aes_128_ecb();
    let key = b"YELLOW SUBMARINE";
    let first_block = decrypt_data_1(&first_block, key);//decrypt(cipher,key,Some(iv2), &first_block.as_slice()).unwrap();
    let first_block_16 = &first_block[0..16];
    let first_block = repeating_xor_encrypt(&first_block_16, &iv);
    
    plaintext.push(first_block);
    let mut j = 0;
    for i in (16.. ciphertext.len()).step_by(16){

        let nth_block = &ciphertext[i..i+16];
        let nth_block = decrypt_data_1(&nth_block, key);
        let nth_block_16 = &nth_block[0..16];
        println!("{}, {}",nth_block_16.len(),plaintext.get(j).unwrap().len());
        let nth_block = repeating_xor_encrypt(&nth_block_16, &ciphertext[i-16..i]);
        
        j +=1;
        plaintext.push(nth_block);
    }

    for i in 0..plaintext.len()
    {
        println!("{}", std::str::from_utf8(plaintext.get(i).unwrap()).unwrap_or("not utf8"));
    }
    
    vec![1]
}

fn repeating_xor_encrypt(plaintext:&[u8], key: &[u8])-> Vec<u8>{
    let mut reapeating_keys = key.iter().cycle();
    plaintext.into_iter().map(|x|x^reapeating_keys.next().unwrap()).collect()
}


fn decrypt_data_1(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut decrypted = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    let mut output = vec![0 as u8; data.len() + Cipher::aes_128_cbc().block_size()];

    let decrypted_result = decrypted.update(&data, &mut output);

    match decrypted_result {
        Ok(_) => output,
        Err(e) => panic!("Error decrypting text: {}", e),
    }
}