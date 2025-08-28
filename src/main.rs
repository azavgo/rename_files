use std::env::{args};
use rand::prelude::*;  

pub fn random_string(n: u8) -> String {
    let random_characters_set = 
        ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', 
        '4', '5', '6', '7', '8', '9'];

    let mut rng = rand::rng();
    let mut rename_set: Vec<char> = vec!['-'];

    for _i in 0..n {
        rename_set.push(random_characters_set[rng.random_range(0..36)]); 
    }
    rename_set.into_iter().collect()
}

fn main() {
    match args().nth(1) {
        Some(file) => {
            let rename_string = random_string(8); 
            let file_renamed = format!("{file}{rename_string}"); 
            println!("{}", file_renamed);}, 
        None => println!("No argument passed"), 
    } 
}
 