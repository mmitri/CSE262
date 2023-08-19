#[derive(Debug, Clone)]
enum Token{
    Lower(u8),
    Upper(u8),
    Dash,
    Period,
    Space,
    Digit,
    Other,
}

fn lex(input: &str) -> Vec<Token>{
    input.chars()
         .map(|char| {
            let char = char as u8;
            match char{
                0x41..=0x5A => Token::Upper(char),
                0x61..=0x7A => Token::Lower(char),
                0x2D => Token::Dash,
                0x2E => Token::Period,
                0x20 => Token::Space,
                _ => Token::Other,
            }
         }).collect()
    vec![]
}

// Parser Combinator

fn digit(mut tokens: Vec<Token>) -> Result<Vec<Token>,Vec<Token>>{
    match token[0]{
        Token::Digit(byte) => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn dash(mut tokens: Vec<Token>) -> Result<Vec<Token>,Vec<Token>>{
    match token[0]{
        Token::Dash(byte) => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn period(mut tokens: Vec<Token>) -> Result<Vec<Token>,Vec<Token>>{
    match token[0]{
        Token::Period(byte) => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn lowercase(mut tokens: Vec<Token>) -> Result<Vec<Token>,Vec<Token>>{
    match token[0]{
        Token::Lower(byte) => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn uppercase(mut tokens: Vec<Token>) -> Result<Vec<Token>,Vec<Token>>{
    match token[0]{
        Token::Upper(byte) => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn space(mut tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>>{
    match token[0]{
        Token::Space => {
            tokens.remove(0);
            Ok(tokens)
        },
        _ => Err(tokens),
    }
}

fn char(mut tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>>{
    match lowercase(tokens.clone()){
        Ok(t) => {return Ok(t);},
        Err(t) => (),
    }
    match uppercase(tokens.clone()){
        Ok(t) => {return Ok(t);},
        Err(t) => (),
    }
    match dash(tokens.clone()){
        Ok(t) => {return Ok(t);},
        Err(t) => (),
    }
    match period(tokens.clone()){
        Ok(t) => {return Ok(t);},
        Err(t) => {return Err(t);},
    }
}

fn first_name(mut tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>>{
    match uppercase(tokens.clone()){
        Ok(t) => {
            let mut t = t;
            loop{
                t = char(t.clone())?;
            }
            Ok(t)
        }
        Err(t) => {return Err(t);},
    }
}

fn last_name(mut tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>>{
    match uppercase(tokens.clone()){
        Ok(t) => {
            let mut t = t;
            loop{
                t = char(t.clone())?;
            }
            Ok(t)
        }
        Err(t) => {return Err(t);},
    }
}

// name = first_name, "space", "last_name";
fn name(mut tokens: Vec<Token>) -> Result<Vec<Token>, Vec<Token>>{
    let tokens = first_name(tokens)?;
    let tokens = space(tokens)?;
    let tokens = last_name(tokens)?;
}

fn parse(input: &str) -> Result<Vec<Token>, Vec<Token>>{
    let mut tokens: Vec<Token> = lex(input);
    tokens = first_name(tokens)?;
    if tokens.len() == 0{
        Ok(tokens)
    }else{
        Err(tokens)
    }
}

fn main() {
    let my_name = "Hello";
    let result = parse(my_name);
    println!("{:#?}", result);
}
