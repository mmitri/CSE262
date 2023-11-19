extern crate nom;
extern crate asalang_parser;

use asalang_parser::{variable_define, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  let result = variable_define(r#"let x = 123;"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:?}", tree);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }
    
  Ok(())
}