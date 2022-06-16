use single_byte_xor::single_byte_xor::find_xor_key;
use std::collections::BTreeMap;

fn main() {
    let plain = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal.";
    let key = "ICE".as_bytes();
    let encrypted = repeating_xor_encrypt(&plain.as_bytes(),&key);
    //println!("{}",hex::encode(repeating_xor_encrypt(&plain.as_bytes(),&key)));
    let possible_lens = find_repeating_xor_keylen(&encrypted);
    // println!("{}", possible_lens.0);
    // println!("{}", possible_lens.1);
    let transposed = transpose(encrypted, possible_lens.0);
    println!("{}", transposed.len());
    for i in 0..transposed.len(){
       let (guessed_key, score) =  find_xor_key(&transposed[i]);
        println!("{}, {}", guessed_key, score);
    }

}

fn repeating_xor_encrypt(plaintext:&[u8], key: &[u8])-> Vec<u8>{
    let mut reapeating_keys = key.iter().cycle();
    plaintext.into_iter().map(|x|x^reapeating_keys.next().unwrap()).collect()
}

fn break_reapeating_xor(ciphertext: &[u8]) {
    let mut min_distance = 0;
}

fn find_repeating_xor_keylen(ciphertext: &[u8]) -> (u64, u64){

    let mut distances_keys = BTreeMap::new();
    let max_keylen = if ciphertext.len()/2 >= 40 {40} else {ciphertext.len()/2};
    for keysize in 3..max_keylen{
        let mut distances = Vec::new();

        let mut chunks_amount = ciphertext.len() / keysize;
        if chunks_amount %2 != 0{
            chunks_amount -= 1;
        }
        let chunk_size = ciphertext.len() / chunks_amount;

        //println!("ciphertext len {}, chunk_amount {}, chunk size: {}",ciphertext.len(),chunks_amount, chunk_size);
        for chunk_index in (0..chunks_amount).step_by(2){
            let chunk1 = &ciphertext[chunk_index *chunk_size ..(chunk_index+1)* chunk_size];
            let chunk2 = &ciphertext[(chunk_index+1)* chunk_size..(chunk_index+2)* chunk_size];
            let distance = hamming::distance(chunk1, chunk2);
            distances.push(distance);
        }
        let average_dist: u64 = (distances.iter().sum::<u64>() as f64 /(distances.len() as f64)).round() as u64;
        //println!("avg dist of {} is {}",keysize, average_dist);
        distances_keys.insert(average_dist,keysize);
    }
    let best_keysize = distances_keys.iter().next().unwrap().1;
    let second_best_keysize = distances_keys.iter().nth(1).unwrap().1;
    (*best_keysize as u64, *second_best_keysize as u64)
}

fn transpose(ciphertext: Vec<u8> ,keysize: u64) -> Vec<Vec<u8>>
{
    let chunks_amount = ciphertext.len() / keysize as usize;

    let mut blocks: Vec<Vec<u8>> = Vec::new();

    for i in 0..keysize{
        let mut temp_block = Vec::<u8>::new();
        for j in (i..ciphertext.len() as u64).step_by(keysize as usize){
            &temp_block.push(ciphertext[j as usize]);
        }
        blocks.push(temp_block);
    }
        println!("{:?}",blocks);
        blocks
}