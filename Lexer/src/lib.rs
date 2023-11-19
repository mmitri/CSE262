/***
 * Mark Mitri
 * CSE262-010-SP23
 * Homework 2 | Asa Lexer
 */

/***
 *  Keyword(Vec<u8>),
    Alpha(u8),
    Digit(u8),

    Token::Keyword(vec![0x74, 0x72, 0x75, 0x65])
    0x41..=0x5A | 0x61..=0x7A => Token::Alpha(bytes[count]),
    0x30..=0x39 => Token::Digit(bytes[count]),
 */

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Keyword,    // 'true','false','fn','return','let' (Vec<u8>)
  Alpha,      // A-Z, a-z (u8)
  Digit,      // 0-9 (u8)
  LeftParen,  // '('
  RightParen, // ')'
  LeftCurly,  // '{'
  RightCurly, // '}'
  Equal,      // '='
  Plus,       // '+'
  Dash,       // '-'
  Quote,      // '"'
  WhiteSpace, // ' '
  Semicolon,  // ';'
  Comma,      // ','
  Other,      // other non-recognized tokens
  EOF,        // End of file will be placed at the end of the token vector
}

pub fn lex(input: &str) -> Vec<Token> {
  let bytes = input.as_bytes(); // container of bytes
  let mut tokens = vec![];
  let mut count = 0;
  while count < bytes.len(){
    let token = match bytes[count]{
      // The idea for the match conditions for the keywords
      // was from https://piazza.com/class/ld8rol0jno394/post/44
      // Check if true
      0x74 if bytes.get(count + 1) == Some(&0x72) && 
      bytes.get(count + 2) == Some(&0x75) && 
      bytes.get(count + 3) == Some(&0x65) => {
        count += 3;
        Token::Keyword
      },
      // Check if false
      0x66 if bytes.get(count + 1) == Some(&0x61) && 
      bytes.get(count + 2) == Some(&0x6C) && 
      bytes.get(count + 3) == Some(&0x73) && 
      bytes.get(count + 4) == Some(&0x65) => {
        count += 4;
        Token::Keyword
      },
      // Check if fn
      0x66 if bytes.get(count + 1) == Some(&0x6E) => {
        count += 1;
        Token::Keyword
      }
      // check if let
      0x6C if bytes.get(count + 1) == Some(&0x65) && 
      bytes.get(count + 2) == Some(&0x74) => {
        count += 2;
        Token::Keyword
      },
      // check if return
      0x72 if bytes.get(count + 1) == Some(&0x65) && 
      bytes.get(count + 2) == Some(&0x74) && 
      bytes.get(count + 3) == Some(&0x75) && 
      bytes.get(count + 4) == Some(&0x72) && 
      bytes.get(count + 5) == Some(&0x6E) => {
        count += 5;
        Token::Keyword
      },
      0x41..=0x5A | 0x61..=0x7A => Token::Alpha,
      0x30..=0x39 => Token::Digit,
      0x28 => Token::LeftParen,
      0x29 => Token::RightParen,
      0x7B => Token::LeftCurly,
      0x7D => Token::RightCurly,
      0x3D => Token::Equal,
      0x2B => Token::Plus,
      0x2D => Token::Dash,
      0x22 => Token::Quote,
      0x20 | 0x17 | 0xA | 0xC | 0xD => Token::WhiteSpace,
      0x3B => Token::Semicolon,
      0x2C => Token::Comma,
      _ => Token::Other,
    };
    tokens.push(token);
    count += 1;
  }
  tokens.push(Token::EOF);
  tokens
}

pub fn strip_whitespace(tokens: &Vec<Token>) -> Vec<Token>{
  let mut new_token:Vec<Token> = vec![];
  for token in tokens{
    if *token != Token::WhiteSpace{
      new_token.push(token.clone());
    }
  }
  new_token
}