use lexer::*;

#[test]
fn test_01() {
  assert_eq!(lex("123"),vec![Token::Digit, Token::Digit, Token::Digit, Token::EOF]);
}

#[test]
fn test_02() {
  assert_eq!(lex("abc"),vec![Token::Alpha, Token::Alpha, Token::Alpha, Token::EOF]);
}

#[test]
fn test_03() {
  assert_eq!(lex("hello world"),vec![Token::Alpha, Token::Alpha, Token::Alpha, Token::Alpha, Token::Alpha, Token::WhiteSpace, Token::Alpha, Token::Alpha, Token::Alpha, Token::Alpha, Token::Alpha, Token::EOF]);
}

#[test]
fn test_04() {
  assert_eq!(lex("true"),vec![Token::Keyword, Token::EOF]);
}

#[test]
fn test_05() {
  assert_eq!(lex("false"),vec![Token::Keyword, Token::EOF]);
}

#[test]
fn test_06() {
  assert_eq!(lex("let x = 123;"),vec![
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha, 
    Token::WhiteSpace,
    Token::Equal,
    Token::WhiteSpace,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Semicolon,
    Token::EOF,
  ]);
}

#[test]
fn test_07() {
  assert_eq!(lex(r#"let x = 123;let y="abc";"#),vec![
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha, 
    Token::WhiteSpace,
    Token::Equal,
    Token::WhiteSpace,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Semicolon,
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha, 
    Token::Equal,
    Token::Quote,
    Token::Alpha, 
    Token::Alpha, 
    Token::Alpha, 
    Token::Quote,
    Token::Semicolon,
    Token::EOF,
  ]);
}

#[test]
fn test_08() {
  assert_eq!(lex(r#"fn main() {}"#),vec![
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha, 
    Token::Alpha,
    Token::Alpha,
    Token::Alpha,
    Token::LeftParen,
    Token::RightParen,
    Token::WhiteSpace,
    Token::LeftCurly,
    Token::RightCurly,
    Token::EOF,
  ]);
}


#[test]
fn test_09() {
  assert_eq!(lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x*y;
}"#),vec![
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha, 
    Token::Alpha,
    Token::Alpha,
    Token::LeftParen,
    Token::Alpha,
    Token::Comma,
    Token::Alpha,
    Token::Comma,
    Token::Alpha,
    Token::RightParen,
    Token::WhiteSpace,
    Token::LeftCurly,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::Keyword, 
    Token::WhiteSpace, 
    Token::Alpha,
    Token::Equal,
    Token::Alpha,
    Token::Plus,
    Token::Digit,
    Token::Semicolon,
    Token::WhiteSpace, 
    Token::WhiteSpace, 
    Token::Keyword, 
    Token::WhiteSpace,
    Token::Alpha,
    Token::Equal,
    Token::Alpha,
    Token::Alpha,
    Token::Alpha,
    Token::LeftParen,
    Token::Alpha,
    Token::Dash,
    Token::Alpha,
    Token::RightParen,
    Token::Semicolon,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::Keyword, 
    Token::WhiteSpace,
    Token::Alpha,
    Token::Other,
    Token::Alpha,
    Token::Semicolon,
    Token::WhiteSpace,
    Token::EOF,
  ]);
}

#[test]
fn test_10() {
  assert_eq!(strip_whitespace(&lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x+y;
}"#)),vec![
    Token::Keyword, 
    Token::Alpha, 
    Token::Alpha,
    Token::Alpha,
    Token::LeftParen,
    Token::Alpha,
    Token::Comma,
    Token::Alpha,
    Token::Comma,
    Token::Alpha,
    Token::RightParen,
    Token::LeftCurly,
    Token::Keyword, 
    Token::Alpha,
    Token::Equal,
    Token::Alpha,
    Token::Plus,
    Token::Digit,
    Token::Semicolon,
    Token::Keyword, 
    Token::Alpha,
    Token::Equal,
    Token::Alpha,
    Token::Alpha,
    Token::Alpha,
    Token::LeftParen,
    Token::Alpha,
    Token::Dash,
    Token::Alpha,
    Token::RightParen,
    Token::Semicolon,
    Token::Keyword, 
    Token::Alpha,
    Token::Plus,
    Token::Alpha,
    Token::Semicolon,
    Token::EOF,
  ]);
}