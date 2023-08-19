/*** 1/23/23
 * 
 * 
***/

/*** 1/30/23
 * 
 * Java
 *  Who: Sun Microsystems (Oracle)
 *  What: An object-oriented programming language
 *  When: 1995
 *  Where: California
 *  Why: To test oop to the extreme!
 *  Users: Workerbees
 *  Major Features:
 *      Major paradigm: OOP
 *      Compiles to JVM bytecode
 *          physical machine is hardware
 *          virtual machine is software
 *              mimic the functionality of a real machine, targeting virtual machines
 *              security, portability, resource management
 *          performance hit
 *      Memory Management: Garbage collected
 *          Do not have to allocate or free memory conciously
 * 
 * Rust
 *  Who: Graydon Hoare (first), Mozilla (second), Rust Foundation (now)
 *  What: A language empowering everyone to build reliable and efficient software
 *  When:
 *  Where:
 *  Why: Memory management is HARD
 *  Major Features:
 *      Zero cost abstractions
 *      Borrow Checker
 *      Cargo toolchain, Crates package manager
 *      Really awesome docs
 *      Type system
*/

/*** 2/1/23
 * 
 * 
***/

/*** 2/6/23
 * Cargo uses subtools to show useful errors
 *  1. Semantic Checker
 *  2. Borrow Checker
 *      Enforce memory safety by default
 * 
 * two kinds of strings
 *  &str - "Hello" [raw string/ string literal]
 *  String - String::new("Hello") [object String] [Use this guy more]
 * 
 * All variables by default are immutable
 * match statements are checked at compile time
 * 
***/

/*** 2/8/23
 * 
 * 
***/

/*** 2/13/23
 * A program is source code (Java, C, HTML) that is easy for humans to comprehend
 * A compiler is a translation machine from source code to machine code, a document that is either
 *      is an executable (run on physical hardware) (compiling)
 *          programs that you can "double-click"
 *      bytecode (run on a virtual machine in software) (compiling)
 *      another language (Transpiling)
 * 
 * Simple Program in source code, saved in a file as a list of bytes, where each byte corresponds to a character:
 *  let x = 123 + 456;
 *  
 *  2^8 (255) possible characters that make up the source file, found in the ASCII table
 * 
***/

/*** 2/15/23
 * 
 * 
***/

/*** 2/20/23
 * EBNF : Extended Backus-Naur Form
 *      Notation that specifies grammer
 *      A series of rules that specifies a sequence or another rule
 * 
 * EX: (555) 555-5555
 * 
 * P := "(",D,D,D,")",D,D,D,"-",D,D,D,D; [Rule for phone number sequence] or
 * P := "(", {D}3,")", {D|"-"};
 * D := "0"..."9"; [Rule for which digits are valid]
 * 
***/

/*** 2/22/23
 * 
 * 
***/

/*** 2/27/23
 * name = first-name, "space", middle-name, "space", last-name;
 * 
 * {} - 1 or more repetition
 * [] - optional
 * () - grouping (max precidence)
 * 
 * first-name = uppercase, {alpha};
 * middle-name = uppercase, {alpha};
 * last-name = uppercase, {alpha};
 * 
 * dash = "-";
 * period = ".";
 * lowercase = "a".."z";
 * uppercase = "A".."Z";
 * char = lowercase | uppercase | dash | period;
 * digits = "0".."9";
 * 
 * function - same input, same output
 * procedure - same input, not necessarily same output
 * 
***/

/*** 3/1/23
 * 
 * 
***/

/*** 3/3/23
 * 
 * 
***/

/*** 3/6/23
 * 
 * 
***/

/*** 3/8/23
 *  parse tree is lossless
 *  source -> [parser] -> parse tree
 *  parse tree -> [formatter] -> source
 *  
 *                          EBNF            Rust
 *  definition              =               function
 *  concatenation           ,               function call, tuple()
 *  termination             ;               function return
 *  alternation             |               alt() [nom combinator]
 *  optional                [...]           opt() [nom]      
 *  repetition              {...}           many1(), many0()
 *  grouping                (...)           (), indirection
 *  terminal string         "..."           "", r#"They said "Hello"  "
 *  comment                 (*...*)         //
 *  special sequence        ?...?           Not a thing.
 *  exception                 -             not()
 * 
***/

/*** 3/13/23
 * 
 * 
***/

/*** 3/15/23
 * 
 * 
***/

/*** 3/20/23
 * 
 * 
***/

/*** 3/22/23
 *  email-address = username, "@", tld, extention;
 *  username = alpha, {alpha-numeric};
 *  tld = alpha, {alpha-numeric};
 *  extention = ".", ("com" | "org");
 * 
 * first class functions:
 *  using a function as a value/variable and used as one.
 *  can pass a function handle to a function as an argument
 * 
 *  EX: Rust
 *      let x = |y: String|{
 *          println!("Hello, {}", y);
 *      };
 *      let y = |y: String|{
 *          println!("Goodbye, {}", y);
 *      };
 *      execute(x);
 *      execute(y);
 * 
 *      fn execute(y: dyn Fn(String)->()){
 *          y("Corey".to_string());
 *      }
 * ==================================================
 * 
 * extern nom;
 * use nom::{
 *      IResult,
 *      character::complete::{alpha0,alphanumeric0},
 *      multi::{many0, many1},
 *      bytes::complete::{tag, take_while_m_n},
 *      combinator::{map_res, opt},
 *      branch::alt,
 *      sequence::tuple
 * };
 * nom = "7.1.3"
 * 
 * #[derive(Debug)]
 * struct Email{
 *      username: String,
 *      domain: String,
 * }
 * 
 * // username = {alpha}, {alpha-numeric};
 * fn username(input: &str) -> IResult<&str, String>{
 *      let(input, alphas) = alpha0(input)?;
 *      let(input, alpha_nums) = alphanumeric0(input)?;
 *      let username = format!("{}{}", alphas,alpha_nums);
 *      Ok((input,username))
 * }
 * 
 * // tld = alpha, {alpha-numeric};
 * fn tld(input: &str) -> IResult<&str, Email>{
 *      let(input, alphas) = alpha0(input)?;
 *      let(input, alpha_nums) = alphanumeric0(input)?;
 *      let tld = format!("{}{}", alphas,alpha_nums);
 *      Ok((input,tld))
 * }
 * 
 * // extention = ".", ("com" | "org");
 * fn extention(input: &str) -> IResult<&str, Email>{
 *      let(input, _) = tag(".")(input)?;
 *      let(input, _) = alt((tag("com"), tag("org"))(input)?; 
 *      Ok((input, ()))
 * }
 * 
 * // email-address = username, "@", tld, extention;
 * fn email_address(input: &str) -> IResult<&str, Email>{
 *      let(input, username) = username(input)?;
 *      let(input, _) = tag("@")(input)?;
 *      let(input, tld) = tld(input)?;
 *      let(input, ext) = extention(input)?;
 *      Ok((input, Email{username, domain: tld});
 * }
 * 
 * fn parse(input: &str) -> IResult<&str, Email>{
 *      email_address(input)
 * }
 * 
 * fn main(){
 *      println!("{:?}", parse("corey@foo.org".to_string()));
 * }
 * 
***/

/*** 3/27/23
 * math = digit, ("+" | "-") , digit;
 * digit = 0-9;
 *      EX: 1 + 1
 * 
 * turn into a tree
 *      digit -> 1 + 1 
 *            -> math 
 *            -> 1 -> + -> 1
 * 
 *      digit -> +
 *            -> 1 -> 1
 * 
 * stack: first in first out
 * tree walking interpreter starts at node, then visits all the children (BFS)
 *      when visited node, pushed to stack
 *      EX: [top] +, 1, 1 [bottom]
 *          1 + 1 = 2
 *          [top] 2 [bottom]
 * 
 * (2 + 3) <- infix
 * (+ 2 3) <- prefix / polish
 * (2 3 +) <- postfix / reverse polish (RPN)
 * 
 * Tree walking interpreter
 *  Input: AST (Abstract syntax tree)
 *  Output: program output
 *      1. start at node
 *      2. to evaluate the root node, evaluate all children, left to right
 *          2a. if the child is a leaf value (bool | string | number | id),
 *              push value onto eval stack
 *          2b. move onto next child
 *      3. when all children are evaluated, pop off the stack, use popped values as operands
 *         into operator. Evaluate
 *      4. push result onto stack
 *      5. program continues until all nodes are visited and evaluated, or an error is encountered.
 *          
 * 
***/

/*** 3/29/23
 * 
 * 
***/

/*** 4/3/23
 * 
 * 
***/

/*** 4/5/23
 * 
 * 
***/

/*** 4/10/23
 * 
 * 
***/

/*** 4/12/23
 * 
 * 
***/

/*** 4/17/23
 * 
 * 
***/

/*** 4/19/23
 * 
 * 
***/

/*** 4/24/23
 * 
 * 
***/

/*** 4/28/23
 * 
 * 
***/