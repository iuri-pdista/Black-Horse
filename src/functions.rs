extern crate crypto;

use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

pub fn substitute(input: String, count: String) -> io::Result<()>{
    let mut file = File::create(count + ".txt")?;
    let mut hasher = Sha512::new();
    println!("{:?}", &input);
    hasher.input_str(&input);
    let hex: String = hasher.result_str();
    let final_string =  hex.into_bytes();
    file.write_all(&final_string)?;
    Ok(())
}