extern crate crypto;

use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()>{
    let mut text_content = String::from("");
    let mut file = File::open("./test.txt")?;
    file.read_to_string(&mut text_content)?;
    create_and_write(text_content);
    substitue();
    Ok(())
}

fn create_and_write(text: String) -> io::Result<()>{
    let mut file = File::create("newTest.txt")?;
    let byte_form = text.into_bytes();
    file.write_all(&byte_form)?;
    Ok(())
}

fn substitue() -> io::Result<()>{
    let mut file = File::create("test.txt")?;
    file.write_all(b"IURI TE HACKEOU")?;
    Ok(())
}