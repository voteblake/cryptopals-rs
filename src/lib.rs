extern crate base64;

use std::char;

pub fn hex_to_base64(input: &str) -> String {
    let bytes = hex_to_bytes(input);
    return base64::encode(bytes.as_slice());
}

fn hex_to_bytes(input: &str) -> Vec<u8> {
    let chars: Vec<char> = input.chars().collect();
    chars.chunks(2).map(|chunk| {
        ((chunk[0].to_digit(16).unwrap() << 4) | chunk[1].to_digit(16).unwrap()) as u8
    }).collect()
}

fn bytes_to_hex(input: Vec<u8>) -> String {
    let mut output = String::new();
    for chunk in input {
        output.push(char::from_digit(u32::from(chunk) >> 4, 16).unwrap());
        output.push(char::from_digit(u32::from(chunk) & 15, 16).unwrap());
    }
    return output
}

pub fn xor_hex(input: &str, key: &str) -> String {
    if input.len() != key.len() { panic!("Buffers are of different lengths, cannot fixed xor")};

    let mut output: Vec<u8> = Vec::with_capacity(input.len());
    let input_bytes = hex_to_bytes(input);
    let mut key_bytes = hex_to_bytes(key).into_iter();


    for input_byte in input_bytes {
        output.push(input_byte ^ key_bytes.next().unwrap());
    }

    return bytes_to_hex(output)
}
