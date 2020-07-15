use std::env;
use std::fs::File;
use std::string::String;
use std::io;
use std::io::prelude::*;

mod functions;

fn main() -> io::Result<()>{
    let sys_args: Vec<String> = env::args().collect();
    let file_path = sys_args[1].clone();
    let mut text_content= String::from("");
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut text_content)?;
    let new_text_content: String = text_content.clone();
    functions::create_and_write(text_content);
    functions::substitute(new_text_content);
    Ok(())
}
