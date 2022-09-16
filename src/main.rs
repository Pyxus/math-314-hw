#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::vec;

const ASCII_A: u8 = 'A' as u8;
const ALPHABET_SIZE: u8 = 26;

fn main() {
    let msg = String::from("TXAENF");
    let msg_indicies = string_to_alphabet_index(&msg);
    let e_indicies = encrypt_affine((4, 13), &msg);
    let e_msg = alphabet_index_to_string(&e_indicies);
    
    println!("{msg}");
    println!("{msg_indicies:?}");
    println!("{e_msg}");
    println!("{e_indicies:?}");
}


fn string_to_alphabet_index(text: &String) -> Vec<u8> {
    let mut index_array: Vec<u8>= Vec::new();

    for c in text.to_uppercase().chars(){
        let text_index = (c as u8) - ASCII_A; 
        index_array.push(text_index);
    }
    index_array
}


fn alphabet_index_to_string(a_indicies: &Vec<u8>) -> String{
    let mut letters = String::new();
    for value in a_indicies{
        letters.push((value + ASCII_A) as char)
    }
    letters
}

fn gcd(mut a: u8, mut b: u8) -> u8 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}


fn mod_inv(a: u8, n: u8) -> u8 {
    if gcd(a, n) == 1 {

    }
    0
}

fn encrypt_affine(key: (u32, u32), msg: &String) -> Vec<u8> {
    let mut encrypted_msg: Vec<u8> = Vec::new();
    let alphabet_index_arr = string_to_alphabet_index(msg);

    for alpha_i in alphabet_index_arr{
        let alpha_i = alpha_i as u32;
        let e_val = (alpha_i * key.0 + key.1) % ALPHABET_SIZE as u32;
        encrypted_msg.push(e_val as u8);
    }
    encrypted_msg
}

fn encrypt_playfair(key: &String, msg: &String){
    
}
/*
fn decrypt_affine(key: (u32, u32), msg: String) -> String{
    let mut decrypted_msg = String::new();
    decrypted_msg
}
*/