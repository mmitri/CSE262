use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]

enum Token{
    Alpha(u8),      // tokens, a-z, A-Z
    Number(u8),     // tokens, 0-9
    Operator(u8),   // tokens, + - / * ^ =
    Grouping(u8),   // tokens, () {} <> [] ||
    Whitespace(u8), // space, \t, \r, \n
    Other(u8),
}

fn main(){
    // Read a source file into our program
    let mut file = File::open("main.asa").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Display source code string
    println!("{}", contents);
    
    // Display source code bytes
    println!("{:?}", contents.as_bytes());

    let bytes = contents.as_bytes(); // container of bytes

    let mut tokens = vec![];
    for byte in bytes{
        let token = match byte{
            0x41..=0x5A | 0x61..=0x7A => Token::Alpha(byte.clone()),
            0x30..=0x39 => Token::Number(byte.clone()),
            0x2A | 0x2B | 0x2F | 0x55 | 0x5E | 0x3D => Token::Operator(byte.clone()),
            0x28 | 0x29 | 0x5B | 0x5D | 0x7B..=0x7D | 0x3C | 0x3E => Token::Grouping(byte.clone()),
            0x20 | 0x9..=0xF => Token::Whitespace(byte.clone()),
            _ => Token::Other(byte.clone()), 
        };
        tokens.push(token);
    }
    println!("{:?}", tokens);
}
