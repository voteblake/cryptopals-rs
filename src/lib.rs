extern crate base64;

use std::char;
use std::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


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

pub fn decrypt_single_char_xor (input: &str, corpus: Option<&HashMap<char, f64>>) -> String {
    let candidates = build_candidate_list(input);
    match corpus {
        Some(x) => return best_candidate_against_corpus(candidates, x),
        None => {
            let local_corpus = build_corpus_from_file("_test_data/205-0.txt");
            return best_candidate_against_corpus(candidates, &local_corpus)
        },
    };
}

fn build_candidate_list (input: &str) -> Vec<Box<str>> {
    let mut results : Vec<Box<str>> = Vec::with_capacity(256);
    for key in 0u8..255u8 {
        let output: Vec<u8> = hex_to_bytes(input).into_iter().map(|byte| {
            byte ^ key
        }).collect();

        results.push(String::from_utf8(output).unwrap_or_default().into_boxed_str());
    }

    return results
}

fn build_corpus (input: &str) -> HashMap<char, f64> {
    let mut output : HashMap<char, f64> = HashMap::new();
    let mut denominator = 0f64;

    for character in input.chars() {
        let count = output.entry(character).or_insert(0f64);
        *count += 1f64;
        denominator += 1f64;
    }
    for value in output.values_mut() {
        *value = *value / denominator
    }
    return output
}

fn best_candidate_against_corpus(candidates: Vec<Box<str>>, corpus: &HashMap<char, f64>) -> String {
    let mut best_score = 0f64;
    let mut best_candidate = 0usize;

    for i in 0..candidates.len() {
        let mut score = 0f64;
        candidates[i].chars().for_each(|candidate_char|{
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

    return candidates[best_candidate].to_string();
}

fn build_corpus_from_file (input: &str) -> HashMap<char, f64> {
    let mut corpus_buff = String::new();
    File::open(input).unwrap().read_to_string(&mut corpus_buff).unwrap();
    build_corpus(&corpus_buff)
}

pub fn bruteforce_single_char_xor(file_path: &str) -> String {

    let mut file = String::new();
    File::open(file_path).unwrap().read_to_string(&mut file).unwrap();

    let corpus = build_corpus_from_file("_test_data/205-0.txt");
    let mut candidates: Vec<Box<str>> = Vec::with_capacity(file.len());

    for line in file.lines() {
        candidates.push(decrypt_single_char_xor(line, Some(&corpus)).into_boxed_str());
    }
    return best_candidate_against_corpus(candidates, &corpus)
}
