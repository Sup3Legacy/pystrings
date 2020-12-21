use std::collections::HashMap;

fn naive(input : &str, pattern : &str) -> usize {
    let input_bytes = input.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let n = input_bytes.len();
    let m = pattern_bytes.len();
    let mut possible;
    let mut occ = 0;
    for i in 0..(n - m + 1) {
        possible = true;
        for j in 0..m {
            if input_bytes[i + j] != pattern_bytes[j] {
                possible = false;
            }
        }
        if possible {
            occ += 1;
        }
    }
    occ
}

struct Trie {
    value: usize,
    children : HashMap<char, Trie>
}

fn construct_trie(trie : &mut Trie, string : &str, value : usize) -> () {
    
}

fn main() {
    let text = String::from("abcd dcba ab ab");
    let pattern = String::from("ab");
    println!("Number of occurences : {}", naive(&text, &pattern));
}
