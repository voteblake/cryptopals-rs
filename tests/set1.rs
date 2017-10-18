extern crate cryptopals;

#[test]
fn solve_challenge_1() {
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
               cryptopals::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}

#[test]
fn solve_challenge_2() {
    assert_eq!("746865206b696420646f6e277420706c6179",
               cryptopals::xor_hex("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"));
}

#[test]
fn solve_challenge_3() {
    println!("{}", cryptopals::decrypt_single_char_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736", None))
}

#[test]
fn solve_challenge_4() {
    println!("{}",cryptopals::bruteforce_single_char_xor("_test_data/4.txt"))
}
