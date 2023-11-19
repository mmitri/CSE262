// Here is where the various combinators are imported. You can find all the combinators here:
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alphanumeric1, digit1},
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
    MathExpression { children: Vec<Node> },
    L0 { children: Vec<Node> },
    L0Infix { children : Vec<Node> },
    L1 { children: Vec<Node> },
    L1Infix { children : Vec<Node> },
    L2 { children: Vec<Node> },
    L2Infix { children : Vec<Node> },
    L3 { children: Vec<Node> },
    L3Infix { children : Vec<Node> },
    L4 { children: Vec<Node> },
    L4Infix { children : Vec<Node> },
    L5 { children: Vec<Node> },
    L5Infix { children : Vec<Node> },
    L6 { children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    Null,
  }
  
  // Here is the grammar, for your reference:
  
  pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
    Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
  }
  
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
  }
  
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Node::Bool{ value: bool_value}))
  }
  
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: string.join("")}))
  }
  
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
  } 
  
  // expanded math expression to handle lhs and rhs of more complex
  // expressions came from https://gitlab.com/mech-lang/syntax/-/blob/main/src/parser.rs
  // from l0-l6 combinators
  
  pub fn l0(input: &str) -> IResult<&str, Node> {
    let (input, l1) = l1(input)?;
    let (input, mut infix) = many0(l0_infix)(input)?;
    let mut math = vec![l1];
    math.append(&mut infix);
    Ok((input, Node::L0{ children: math }))
  }

  pub fn l0_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l1) = l1(input)?;
    Ok((input, Node::L0Infix{ children: vec![l1] }))
  }

  pub fn l1(input: &str) -> IResult<&str, Node> {
    let (input, l2) = l2(input)?;
    let (input, mut infix) = many0(l1_infix)(input)?;
    let mut math = vec![l2];
    math.append(&mut infix);
    Ok((input, Node::L1{ children: math }))
  }

  pub fn l1_op(input: &str) -> IResult<&str, Node>{
    let (input, op) = alt((tag("+"), tag("-")))(input)?;
    l1(op)
  }

  pub fn l1_infix(input: &str) -> IResult<&str, Node>{
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, op) = l1_op(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l2) = l2(input)?;
    Ok((input, Node::L1Infix{ children: vec![op, l2]}))
  }

  pub fn l2(input: &str) -> IResult<&str, Node> {
    let (input, l3) = l3(input)?;
    let (input, mut infix) = many0(l2_infix)(input)?;
    let mut math = vec![l3];
    math.append(&mut infix);
    Ok((input, Node::L2{ children: math }))
  }

  pub fn l2_op(input: &str) -> IResult<&str, Node>{
    let (input, op) = alt((tag("*"), tag("/")))(input)?;
    l2(op)
  }

  pub fn l2_infix(input: &str) -> IResult<&str, Node>{
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, op) = l2_op(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l3) = l3(input)?;
    Ok((input, Node::L2Infix{ children: vec![op, l3]}))
  }

  pub fn l3(input: &str) -> IResult<&str, Node> {
    let (input, l4) = l4(input)?;
    let (input, mut infix) = many0(l3_infix)(input)?;
    let mut math = vec![l4];
    math.append(&mut infix);
    Ok((input, Node::L3{ children: math }))
  }

  pub fn l3_op(input: &str) -> IResult<&str, Node>{
    let (input, op) = tag("^")(input)?;
    l3(op)
  }

  pub fn l3_infix(input: &str) -> IResult<&str, Node>{
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, op) = l3_op(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l4) = l4(input)?;
    Ok((input, Node::L3Infix{ children: vec![op, l4]}))
  }

  pub fn l4(input: &str) -> IResult<&str, Node> {
    let (input, l5) = l5(input)?;
    let (input, mut infix) = many0(l4_infix)(input)?;
    let mut math = vec![l5];
    math.append(&mut infix);
    Ok((input, Node::L4{ children: math }))
  }

  pub fn l4_op(input: &str) -> IResult<&str, Node>{
    let (input, op) = alt((tag("&"), tag("|")))(input)?;
    l4(op)
  }

  pub fn l4_infix(input: &str) -> IResult<&str, Node>{
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, op) = l4_op(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l5) = l5(input)?;
    Ok((input, Node::L4Infix{ children: vec![op, l5]}))
  }

  pub fn l5(input: &str) -> IResult<&str, Node> {
    let (input, l6) = l6(input)?;
    let (input, mut infix) = many0(l5_infix)(input)?;
    let mut math = vec![l6];
    math.append(&mut infix);
    Ok((input, Node::L5{ children: math }))
  }

  pub fn l5_op(input: &str) -> IResult<&str, Node>{
    let (input, op) = alt((tag("<"), tag(">")))(input)?;
    l5(op)
  }

  pub fn l5_infix(input: &str) -> IResult<&str, Node>{
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, op) = l5_op(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, l6) = l6(input)?;
    Ok((input, Node::L5Infix{ children: vec![op, l6]}))
  }

  pub fn l6(input: &str) -> IResult<&str, Node> {
    let (input, l6) = parenthetical_expression(input)?;
    Ok((input, Node::L6{ children: vec![l6] }))
  }

  pub fn parenthetical_expression(input: &str) -> IResult<&str, Node>{
    let (input, _) = tag("(")(input)?;
    let (input, l0) = l0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, l0))
  }

  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    let (input, l0) = l0(input)?;
    Ok((input, Node::MathExpression{ children: vec![l0] }))   
  }
  
  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((boolean, function_call, number, string, identifier, math_expression))(input)?;
    Ok((input, Node::Expression{ children: vec![result]}))   
  }
  
  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\t"))))(input)?;
    let (input, result) = alt((variable_define, function_return))(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    Ok((input, Node::Statement{ children: vec![result]}))   
  }
  
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?;
    let (input, return_value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
  }
  
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
  }
  
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, arg) = expression(input)?;
    let (input, mut others) = many0(other_arg)(input)?;
    let mut args = vec![arg];
    args.append(&mut others);
    Ok((input, Node::FunctionArguments{children: args}))
  }
  
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?;
    expression(input)
  }
  
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("fn ")(input)?;
    let (input, function_name) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    let (input, mut statements) = many1(statement)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?;
    let mut children = vec![function_name];
    children.append(&mut args);
    children.append(&mut statements);
    Ok((input, Node::FunctionDefine{ children: children }))   
  }
  
  // program = function_definition+ ;
  pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many1(alt((function_definition,function_call,function_return,statement,variable_define,math_expression,expression,number,identifier,boolean,string)))(input)?;
    Ok((input, Node::Program{ children: result}))
  }
  