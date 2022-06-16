use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //purpously no input validation here
    let ciphertext = "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";

    let ciphertext_bytes = hex::decode(ciphertext).expect("failed to decode ciphertext");

    let (xor_key, score) = find_xor_key(&ciphertext_bytes);

    
    detect_xor();
    println!("{}", ciphertext_bytes.iter().map(|x|(x^xor_key) as char ).collect::<String>());
    //println!("{}", score);


}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn xor_encrypt(plaintext: &[u8], xor_key: u8) -> Vec<u8> 
{
    let ciphertext:Vec<u8> = plaintext.iter().map(|x|x^xor_key).collect();
    ciphertext
}

fn detect_xor(){
    if let Ok(lines_iter) = read_lines("C:\\users\\royco\\onedrive\\Documents\\file_1"){
        let mut max_score = 0;
        let mut correct_line = 0;
        let mut xor_key = 0;
        for (i, line) in lines_iter.enumerate(){
            if let Ok(line_safe) = line
            {
                let (xor_key,score) = find_xor_key(&hex::decode(line_safe).unwrap());
                if score > max_score{
                    max_score = score;
                    correct_line = i;
                }
            }
            
        }
       // println!("{} {}", max_score, correct_line);
    }
}

fn find_xor_key(ciphertext: &[u8]) -> (u8, i8){
    
/*
    find the xor key used to encrypt the given ciphertext.
*/
    let mut max_score = 0;
    let mut best_key = 0;
    for xor_key in 1..255{
        let xored_vec = ciphertext.iter().map(|x|x^xor_key).collect(); 
        let score = get_score(xored_vec);
        if score > max_score{
            max_score = score;
            best_key = xor_key;
        } 
    }
    
    (best_key, max_score)
}

fn get_score(text: Vec<u8>) -> i8
/*
    get the score of a suspected plaintext
*/
{
    let text_len = text.len();
    for byte in text.iter(){
        if !byte.is_ascii(){
            return 0;
        }
    }
    let mut score = 1;
    let frequencies: HashMap<char, f32> = calculate_frequencies(&String::from_utf8(text).unwrap());

    for (i,(letter, frequency)) in frequencies.iter().enumerate(){
        if let Some(ideal_frequency) = IDEAL_FREQUENCIES.get(letter) {
            if(frequency-ideal_frequency).abs()<= 10.0{
            score += 1;
            } 
        }
        if *letter == ' '{
            if i >= 3  && i <= text_len -3{    
                score += 3;
            }
            else{
                score -= 3;
            }
        }

    }
    score
}


fn calculate_frequencies(text: &str) -> HashMap<char, f32> {
    let count_to_frequency_scale = 100.0 / text.len() as f32;

    let mut counts = HashMap::new();
    for c in text.chars() {
        *counts.entry(c.to_ascii_uppercase()).or_insert(0.0) += 1.0;
    }

    // Rescale counts to frequencies
    for count in counts.values_mut() {
        *count *= count_to_frequency_scale;
    }
    counts
}

static  IDEAL_FREQUENCIES: Lazy<HashMap<char, f32>> = Lazy::new(|| [
    
    ('E', 11.1607),
    ('M', 3.0129),
    ('A', 8.4966),
    ('H', 3.0034),
    ('R', 7.5809),
    ('G', 2.4705),
    ('I', 7.5448),
    ('B', 2.0720),
    ('O', 7.1635),
    ('F', 1.8121),
    ('T', 6.9509),
    ('Y', 1.7779),
    ('N', 6.6544),
    ('W', 1.2899),
    ('S', 5.7351),
    ('K', 1.1016),
    ('L', 5.4893),
    ('V', 1.0074),
    ('C', 4.5388),
    ('X', 0.2902),
    ('U', 3.6308),
    ('Z', 0.2722),
    ('D', 3.3844),
    ('J', 0.1965),
    ('P', 3.1671),
    ('Q', 0.1962),
        
        ].iter().cloned().collect());
    