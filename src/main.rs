// IN PROGRESS: WORKING ON HASHING A STRING OF RANDOM BITS.
// Simple key pair generator
use rand::prelude::*;
use sha2::{Digest, Sha256};
use sha2::digest::Update;
// use sha2::digest::Update;


fn main() {
    let mut rng = rand::thread_rng();

    //defining the number of bits I want in my string
    // choosing 58 just because.
    let num_bits = 58;

    // converting bits into bytes (dividing bits by 8 and rounding up to the next whole number)
    let num_bytes = (num_bits as f64 / 8.0).ceil() as usize;

    // Allocate memory to store our bytes in a vector
    let mut random_bytes = vec![0u8; num_bytes];

    // Filling random_bytes from the rand crate
    // Not ideal source of entropy but this is just for practise
    rng.fill_bytes(&mut random_bytes);


    let mut binary_string = String::new();

    // iter over each byte in the random_bytes and format into it's binary representation with
    // leading zeros making sure each byte is 8 bits
    for byte in random_bytes {
        binary_string.push_str(&*format!("{:08b}", byte));
    }

    // So I tried to hash the string but the docs used b"string" so I'll try converting it back
    // But It still returns an array.
    let binary_data: Vec<u8> = binary_string
        .chars()// Iterate over each character in the string
        .map(|c| c.to_digit(2).unwrap() as u8) // Convert character to binary digit (0 or 1)
        .collect();


    // truncating the random binary_string to keep out desired length
    binary_string.truncate(num_bits);
    println!("Random binary string ({} bits): {}", num_bits, binary_string);

    // Now time to hash
    let mut hasher = Sha256::new();
    // hasher.update(&binary_string);
    hasher.update(&binary_data);
    let result_hash = hasher.finalize();

    // In progress
    // Why is the result hash an array?
    // For help/docs
    // https://github.com/RustCrypto/hashes?tab=readme-ov-file
    println!("The hash of the binary string is {:?}", result_hash);

}
