extern crate cryptopals;

#[test]
fn solve_challenge_1() {
    assert_eq!(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        cryptopals::hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        )
    );
}

#[test]
fn solve_challenge_2() {
    assert_eq!(
        "746865206b696420646f6e277420706c6179",
        cryptopals::xor_hex(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        )
    );
}

#[test]
fn solve_challenge_3() {
    println!(
        "{}",
        cryptopals::decrypt_single_char_xor(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            None,
        ).unwrap_or_default()
    )
}

#[test]
fn solve_challenge_4() {
    println!(
        "{}",
        cryptopals::bruteforce_single_char_xor("_test_data/4.txt")
    )
}

#[test]
fn solve_challenge_5() {
    assert_eq!(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        cryptopals::encrypt_repeating_key_xor(
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
            "ICE",
        )
    );
    assert_eq!(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        cryptopals::decrypt_repeating_key_xor(
            cryptopals::encrypt_repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE",
            ).as_str(),
            "ICE",
        )
    )
}
