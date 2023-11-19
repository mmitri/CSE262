extern crate nom;
extern crate asalang;

use asalang::{program, Runtime, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"fn main(){return foo();} fn foo(){return 5;}"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      let mut runtime = Runtime::new();
      let result = runtime.run(&tree);
      println!("{:?}", result);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }

    
  Ok(())
}
