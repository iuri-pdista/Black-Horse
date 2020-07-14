use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;


mod functions;

fn main() -> io::Result<()>{
    let mut text_content= String::from("");
    let new_text_content = String::from("HACKEADO PELO IURI");
    let mut file = File::open("./test.txt")?;
    file.read_to_string(&mut text_content)?;
    functions::create_and_write(text_content);
    functions::substitue(new_text_content);
    Ok(())
}
