extern crate crypto;

use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

fn main() -> io::Result<()>{
    let mut text_content= String::from("");
    let new_text_content = String::from("HACKEADO PELO IURI");
    let mut file = File::open("./test.txt")?;
    file.read_to_string(&mut text_content)?;
    create_and_write(text_content);
    substitue(new_text_content);
    Ok(())
}

fn create_and_write(text: String) -> io::Result<()>{
    let mut file = File::create("newTest.txt")?;
    let byte_form = text.into_bytes();
    file.write_all(&byte_form)?;
    Ok(())
}

fn substitue(input: String) -> io::Result<()>{
    let mut file = File::create("test.txt")?;
    let mut hasher = Sha512::new();
    hasher.input_str(&input);
    let hex: String = hasher.result_str();
    let final_string =  hex.into_bytes();
    file.write_all(&final_string)?;
    Ok(())
}