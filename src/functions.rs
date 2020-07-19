extern crate crypto;

use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

pub fn substitute(input: String, count: String) -> io::Result<()>{
    let mut file = File::create(count + ".txt")?;
    let hex = encrypt(input);
    let final_string =  hex.into_bytes();
    file.write_all(&final_string)?;
    Ok(())
}

fn encrypt (input: String) -> String{
    let mut hash = Sha512::new();
    hash.input_str(&input);
    let hex: String = hash.result_str();
    hex
}