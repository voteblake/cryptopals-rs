extern crate base64;

pub fn hex2base64(input: &str) -> String {
    let bytes = hex2bytes(input);
    return base64::encode(bytes.as_slice());
}

fn hex2bytes(input: &str) -> Vec<u8> {
    let chars: Vec<char> = input.chars().collect();
    chars.chunks(2).map(|chunk| {
        ((chunk[0].to_digit(16).unwrap() << 4) | chunk[1].to_digit(16).unwrap()) as u8
    }).collect()
}
