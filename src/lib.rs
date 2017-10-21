extern crate base64;

use std::char;
use std::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


pub fn hex_to_base64(input: &str) -> String {
    let bytes = hex_to_bytes(input);
    base64::encode(bytes.as_slice())
}

fn hex_to_bytes(input: &str) -> Vec<u8> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .chunks(2)
        .map(|chunk| {
            ((chunk[0].to_digit(16).unwrap() << 4) | chunk[1].to_digit(16).unwrap()) as u8
        })
        .collect()
}

fn bytes_to_hex(input: Vec<u8>) -> String {
    let mut output = String::new();
    for chunk in input {
        output.push(char::from_digit(u32::from(chunk) >> 4, 16).unwrap());
        output.push(char::from_digit(u32::from(chunk) & 15, 16).unwrap());
    }
    output
}

pub fn xor_hex(input: &str, key: &str) -> String {
    if input.len() != key.len() {
        panic!("Buffers are of different lengths, cannot fixed xor")
    };

    let mut output: Vec<u8> = Vec::with_capacity(input.len());
    let input_bytes = hex_to_bytes(input);
    let key_bytes = hex_to_bytes(key);


    for i in 0..input_bytes.len() {
        output.push(input_bytes[i] ^ key_bytes[i]);
    }

    bytes_to_hex(output)
}

pub fn decrypt_single_char_xor(input: &str, corpus: Option<&HashMap<char, f64>>) -> Option<String> {
    let candidates = build_candidate_list(input);
    if candidates.is_empty() {
        return None;
    }

    match corpus {
        Some(x) => best_candidate_against_corpus(&candidates, x),
        None => {
            let local_corpus = build_corpus_from_file("_test_data/205-0.txt");
            best_candidate_against_corpus(&candidates, &local_corpus)
        }
    }
}

fn build_candidate_list(input: &str) -> Vec<Box<str>> {
    let mut results: Vec<Box<str>> = Vec::with_capacity(256);
    for key in 0u8..255u8 {
        let output: Vec<u8> = hex_to_bytes(input)
            .into_iter()
            .map(|byte| byte ^ key)
            .collect();


        match String::from_utf8(output) {
            Ok(x) => results.push(x.into_boxed_str()),
            Err(_) => continue,
        }
    }

    results
}

fn build_corpus(input: &str) -> HashMap<char, f64> {
    let mut output: HashMap<char, f64> = HashMap::new();
    let mut denominator = 0f64;

    for character in input.chars() {
        let count = output.entry(character).or_insert(0f64);
        *count += 1f64;
        denominator += 1f64;
    }
    for value in output.values_mut() {
        *value /= denominator
    }
    output
}

fn best_candidate_against_corpus(candidates: &[Box<str>], corpus: &HashMap<char, f64>) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }
    let mut best_score = 0f64;
    let mut best_candidate = 0usize;

    for (i, candidate) in candidates.iter().enumerate() {
        let mut score = 0f64;
        candidate.chars().for_each(|candidate_char| {
            score += match corpus.get(&candidate_char) {
                Some(x) => *x,
                None => 0f64,
            }
        });

        if score > best_score {
            best_score = score;
            best_candidate = i;
        }
    }

    Some(candidates[best_candidate].to_string())
}

fn build_corpus_from_file(input: &str) -> HashMap<char, f64> {
    let mut corpus_buff = String::new();
    File::open(input)
        .unwrap()
        .read_to_string(&mut corpus_buff)
        .unwrap();
    build_corpus(&corpus_buff)
}

pub fn bruteforce_single_char_xor(file_path: &str) -> String {

    let mut file = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut file)
        .unwrap();

    let file_iter = file.lines();

    let corpus = build_corpus_from_file("_test_data/205-0.txt");
    let mut candidates: Vec<Box<str>> = Vec::with_capacity(file_iter.size_hint().1.unwrap_or_default());

    for line in file_iter {
        match decrypt_single_char_xor(line, Some(&corpus)) {
            Some(x) => candidates.push(x.into_boxed_str()),
            None => continue,
        }

    }
    best_candidate_against_corpus(&candidates, &corpus).unwrap_or_default()
}
