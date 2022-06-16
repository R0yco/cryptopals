extern crate hex;

fn main() {
    println!("{}", fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"))
}

fn fixed_xor(input_1: &str, input_2: &str) -> String{
    let input_1 = hex::decode(input_1).expect("invalid hex ");
    let input_2 = hex::decode(input_2).expect("invalid hex ");
    assert_eq!(input_1.len(), input_2.len());
    let xor_result: Vec<u8> = input_1.iter().zip(input_2.iter()).map(|(&a, &b)|a^b).collect();
    hex::encode(xor_result)
}