use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

pub struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    match node {
      Node::Program{children} => {
        for n in children {
          match n {
            Node::FunctionDefine{..} => {
              self.run(n);
            },
            Node::Expression{..} => {
              self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
            },
            Node::Statement{..} => {
              self.functions.insert("main".to_string(), vec![n.clone()]);
            },
            Node::Number{value} => {
              print!("Entered Node::Number...");
              return Ok(Value::Number(*value));
            },
            Node::FunctionCall{name, children} => {
              let statements = self.functions.get(name);
              // for s in statements{
              //   let result = self.start_interpreter(s);
              // }
            },
            Node::VariableDefine{children} => {
              let temp;
              match &children[0]{
                Node::String{value} => temp = value.to_string(),
                x => {
                  return Err("Unparsed Variable input");
                },
              };
              return Ok(Value::String(temp));
            },
            x => todo!(),
          }
        }
        Ok(Value::Bool(true))
      },
      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{children} => {
        Err("MathExpression Int. Unimplemeneted")
      },
      // Defines a function that takes some arguments and executes a program based on those arguments. The code first checks if the function exists, and if it does, it creates a new scope in which to execute the function's statements. The code then executes each statement in the function's statements list and returns the result of the function's execution.
      Node::FunctionCall{name, children} => {
        if let Some(statements) = self.functions.get(name){
          let mut val = Value::Bool(true);
          let temp = HashMap::new();
          self.stack.push(temp);
          for state in statements.clone() {
              val = self.run(&state)?;
          }
          if !self.stack.is_empty() {
              self.stack.pop();
          }
          Ok(val)
          }else{
            Err("Undefined Function Call...")
        }
      },
      // Defines a new function based on the elements in the children argument. The name of the function is retrieved from the first element of the children, and the statements that define the function are retrieved from rest of hte children (head/tail). A new key-value pair is then inserted into the functions field of the current runtime object. If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{children} => {
        // Err("FunctionDefine Int. Unimplemeneted")
        if let Node::String { value } = &children[0] {
          let statements = &children[1..];
          self.functions.insert(value.to_string(), statements.to_vec());
          Ok(Value::Bool(true))
        } else {
            Err("Undefined function")
        }
      },
      // Calls the run method on the first element in the children argument, which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        self.run(&children[0])
      },
      // Retrieves the value of a variable from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        Err("Identifier Int. Unimplemeneted")
      },
      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        Err("Statement Int. Unimplemeneted")
      },
      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        let temp = Default::default();
        match &children[0]{
          Node::Identifier{value} => {
            self.stack.last_mut().unwrap().insert(temp,Value::String(value.to_string()));
          }
          x => {
            return Err("Undefined Variable...")
          },
        };
        
        Ok(Value::Bool(true))
      }
      Node::Expression{children} => {
        Err("Expression Int. Unimplemeneted")
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }
      Node::String{value} => {
        Ok(Value::String(value.to_string()))
      }
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }
      x => {
        Err("Generally Unimplemented ;(")
      },
    }
  }
}

pub fn start_interpreter(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  runtime.run(node);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}
