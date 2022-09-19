use std::{vec, collections::HashMap, cmp::{self, min}};

const ASCII_A: u8 = 'A' as u8;
const ALPHABET_SIZE: u8 = 26;

fn main() {
    //println!("{}", alphabet_index_to_string(vec![13, 20, 12, 1, 4, 17, 19, 7, 4, 14, 17, 24]));
    //let v = string_to_alphabet_index(String::from("ljlljltjunjuuj"));
    let msg = String::from("lcyfqpngcytnkvveakkynktfswdyrnaeerqqoccmhspfshaekrziuvleiwpyyazinwaevydfizcydf
    idzejvloairlooddcdsrxceokycalrechhztfpntiwatjkfpmvjqlgvrseodrrtcrcjjaeuqpnujge
    brtizncpyqrzvlolprgcpcrlpijrqdudvbeoyrtptyvazrivaekvpazmgrpptyvkpsjrepfifkehvg
    jlnvngehkycnoiicntcpcycipneeudcdsrxctfkycjmrkastyvnwaevgdfizcydcpgqnfkgeskycpn
    vdwsonvtprkycpnvdwnaejcydrcycgvesxbvimqcyfqpndvqdaxvqeofeczfpfscpcrlpsrebwofby
    etyvppslcrtnxtgahvirpxkjgqtyzqllcfudtyvkeouvbfcvkfpkvprsevecxytrlpqlznehvzpalr
    ecdsfkfpytrlxajhsprruclswigpnucw");
    let count_dict = char_dist(&msg);
    println!("{count_dict:?}");
    println!("{}", match_count(&String::from("lcyfqpngcytnkvveakkynktfswdyrnaeerqqoccmhspfshaekrziuvleiwpyyazinwaevydfizcydf
    idzejvloairlooddcdsrxceokycalrechhztfpntiwatjkfpmvjqlgvrseodrrtcrcjjaeuqpnujge
    brtizncpyqrzvlolprgcpcrlpijrqdudvbeoyrtptyvazrivaekvpazmgrpptyvkpsjrepfifkehvg
    jlnvngehkycnoiicntcpcycipneeudcdsrxctfkycjmrkastyvnwaevgdfizcydcpgqnfkgeskycpn
    vdwsonvtprkycpnvdwnaejcydrcycgvesxbvimqcyfqpndvqdaxvqeofeczfpfscpcrlpsrebwofby
    etyvppslcrtnxtgahvirpxkjgqtyzqllcfudtyvkeouvbfcvkfpkvprsevecxytrlpqlznehvzpalr
    ecdsfkfpytrlxajhsprruclswigpnucw"), &String::from(" lcyfqpngcytnkvveakkynktfswdyrnaeerqqoccmhspfshaekrziuvleiwpyyazinwaevydfizcydf
    idzejvloairlooddcdsrxceokycalrechhztfpntiwatjkfpmvjqlgvrseodrrtcrcjjaeuqpnujge
    brtizncpyqrzvlolprgcpcrlpijrqdudvbeoyrtptyvazrivaekvpazmgrpptyvkpsjrepfifkehvg
    jlnvngehkycnoiicntcpcycipneeudcdsrxctfkycjmrkastyvnwaevgdfizcydcpgqnfkgeskycpn
    vdwsonvtprkycpnvdwnaejcydrcycgvesxbvimqcyfqpndvqdaxvqeofeczfpfscpcrlpsrebwofby
    etyvppslcrtnxtgahvirpxkjgqtyzqllcfudtyvkeouvbfcvkfpkvprsevecxytrlpqlznehvzpalr
    ecdsfkfpytrlxajhsprruclswigpnucw")));
}


fn string_to_alphabet_index(text: &String) -> Vec<u8> {
    let mut index_array: Vec<u8>= Vec::new();

    for c in text.to_uppercase().chars(){
        let text_index = (c as u8) - ASCII_A; 
        index_array.push(char_to_alphabet_index(&c));
    }
    index_array
}

fn char_to_alphabet_index(c: &char) -> u8{
    (*c as u8) - ASCII_A
}

fn alphabet_index_to_string(a_indicies: &Vec<u8>) -> String{
    let mut letters = String::new();
    for value in a_indicies{
        letters.push(alphabet_index_to_char(value))
    }
    letters
}

fn alphabet_index_to_char(i: &u8) -> char{
    (*i + ASCII_A) as char
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

/*
fn decrypt_affine(key: (u32, u32), msg: String) -> String{
    let mut decrypted_msg = String::new();
    decrypted_msg
}
*/

fn decrypt_vigenere(key: &String, msg: &String) -> String{
    let mut decrypted_msg = String::new();
    let key_len = key.chars().count();
    
    for (i, letter) in msg.to_uppercase().char_indices(){
        let letter_index = char_to_alphabet_index(&letter);
        let key_letter_index = char_to_alphabet_index(&key.chars().nth(i % key_len).unwrap());
        println!("{} - {} = {}", letter_index, ((key_letter_index) % ALPHABET_SIZE), alphabet_index_to_char(&((letter_index + (26 - key_letter_index % ALPHABET_SIZE)) % ALPHABET_SIZE)));
        let letter_index = (letter_index + (26 - key_letter_index % ALPHABET_SIZE)) % ALPHABET_SIZE;

        decrypted_msg.push(alphabet_index_to_char(&letter_index));
    }

    decrypted_msg
}


fn char_count(c: &char, text: &String) -> u8
{
    let mut count = 0;
    for l in text.chars(){
        if *c == l {
            count += 1;
        }
    }
    count
}


fn char_dist(text: &String) -> HashMap<char, u8>{
    let mut dict = HashMap::new();
    for c in text.chars(){
        if !dict.contains_key(&c){
            dict.insert(c, char_count(&c, &text));
        }
    }
    dict
}

fn match_count(text1: &String, text2: &String) -> u8{
    let mut count = 0;
    let it = min(text1.chars().count(), text2.chars().count());
    for i in 0..it {
        if text1.chars().nth(i).unwrap() == text2.chars().nth(i).unwrap(){
            count += 1;
        }
    }
    count
}