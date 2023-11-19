// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  IResult,
  branch::alt,
  combinator::{opt, not, map},
  multi::{many1, many0},
  bytes::complete::{tag},
  character::complete::{digit1, alpha1, space0, alphanumeric1, alpha0, alphanumeric0}, 
  sequence::{preceded, terminated, pair}
};

// Here are the different node types. You will use these to make your parser and your grammar.
// You may add other nodes as you see fit, but these are expected by the runtime.

#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
  Value {value: String},
  Argument { children: Vec<Node> }
}

// Here is the grammar, for your reference:

//identifier = alpha , {alnum} ;
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, alpha) = alpha1(input)?;
  let (input, alphanum) = alphanumeric0(input)?;
  Ok((input, Node::Identifier{value: (alpha.to_owned()+alphanum).to_string()}))
}

// number = digit+ ;
pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, digits) = digit1(input)?;
  let val = digits.parse::<i32>().unwrap(); // Convert string to i32
  Ok((input, Node::Number{value: val}))
}

// boolean = "true" | "false" ;
pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, bool_inp) = alt((tag("false"),tag("true")))(input)?;
  let truth_value: bool = match bool_inp{
    "true" => true,
    "false" => false,
    _ => unreachable!(), 
  };
  Ok((input, Node::Bool { value: truth_value}))
}

// string = "\"" , {alnum | " "} , "\"" ;
pub fn string(input: &str) -> IResult<&str, Node> {
  // match for "\""
  let (input, _) = tag("\"")(input)?;
  // then match for a repeating alphanumeric OR " "
  let (input, _) = many0(pair(alphanumeric1, many0(tag(" "))))(input)?;
  // then match for "\""
  let (input, _) = tag("\"")(input)?;
  Ok((input, Node::String{value: input.to_string()}))
}

// function_call = identifier , "(" , [arguments] , ")" ;
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, ident) = identifier(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, func_child) = opt(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  // case to handle if arguments return an empty type
  let func_children = if let Some(child) = func_child{
    vec![child]
  }else{
    vec![]
  };
  Ok((input, Node::FunctionCall{name: format!("{:?}", ident), children: func_children}))
}

// value = number | identifier | boolean ; ;
pub fn value(input: &str) -> IResult<&str, Node> {
  let (input, value) = alt((number, identifier, boolean))(input)?;
  Ok((input, Node::Value{value: format!("{:?}", value)}))
}

// math_expression = value , { ("+" | "-") , value } ;
pub fn math_expression(input: &str) -> IResult<&str, Node> {
  let (input, first_value) = value(input)?;
  let (input, _) = space0(input)?;
  let (input, child) = many1(preceded(tag("+"), value).or(preceded(tag("-"), value)))(input)?;
  let (input, _) = space0(input)?;
  Ok((input, Node::MathExpression{name: format!("{:?}", first_value), children: child}))
}

// expression = boolean | math_expression | function_call | number | string | identifier ;
pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, express_child) = alt((boolean, math_expression, function_call, number, string, identifier))(input)?;
  let children = vec![express_child];
  Ok((input, Node::Expression{children: children}))
}

// statement = variable_define , ";" | function_return , ";" ;
pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = space0(input)?;
  let (input, variable_child) = variable_define(input)?;
  let (input, _) = tag(";")(input)?;
  let (input, statement_child) = function_return(input)?;
  let (input, _) = tag(";")(input)?;
  let children = vec![variable_child,statement_child];
  Ok((input, Node::Statement{children: children}))
}

// function_return = "return" , (function_call | expression | identifier) ;
pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = space0(input)?;
  let (input, _) = tag("return ")(input)?;
  let (input, function_return_child) = alt((function_call, expression, identifier))(input)?;
  let children = vec![function_return_child];
  Ok((input, Node::FunctionReturn{children: children}))
}

// variable_define = "let" , identifier , "=" , expression ;
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  let children = vec![variable, expression];
  Ok((input, Node::VariableDefine{children: children}))
}

// arguments = expression , { "," , expression } ;
pub fn arguments(input: &str) -> IResult<&str, Node> {
  let (input, express) = expression(input)?;
  let (input, mut arg_child) = many0(other_arg)(input)?;
  let mut args = vec![express];
  args.append(&mut arg_child);
  Ok((input, Node::Argument{children: args}))
}

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag(",")(input)?;
  expression(input)
}

// function_definition = "fn" , identifier , "(" , [arguments] , ")" , "{" , [statement+] , "}" ;
pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("fn ")(input)?;
  let (input, func_def_iden) = identifier(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, mut func_arg) = many0(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = space0(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = space0(input)?;
  let (input, mut func_state) = many1(statement)(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
  let mut children = vec![func_def_iden];
  children.append(&mut func_arg);
  children.append(&mut func_state);
  Ok((input, Node::FunctionDefine{children: children}))
}

// comment = "//" , (?any-character? - newline);
pub fn comment(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("//")(input)?;
  let (input, _) = not(tag("\n"))(input)?;
  Ok((input, Node::String{value: input.to_string()}))
}

// program = function_definition+ ;
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, program) = many1(function_definition)(input)?;
  Ok((input, Node::Program{children: program}))
}
