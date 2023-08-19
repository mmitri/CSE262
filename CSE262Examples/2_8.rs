#[derive(Debug)]
enum Fruit{
    Apple,
    Banana,
    Orange,
    Pear,
}

// Result<T,R> [an interacting-with-the-world error]
// - a special enum type for expressing an operation
//   that can potentially fail
// - examples: opening a file, credentialed access, network communication [crossing a boundary that you don't control everything]
//   wrap within error to leverage pattern matching, reliably producing software that avoids panicks
// - Ok(T)
// - Err(R)

fn main(){
    let input = "Grape";
    let x = Fruit::Apple;
    let res_Fruit: Result <Fruit,String> = match input{
        "Apple" => Ok(Fruit::Apple),
        "Banana" => Ok(Fruit::Banana),
        "Pear" => Ok(Fruit::Pear),
        x => Err(format!("{} is not an approved fruit!", x)),
    };

    println!("{:?}", res_Fruit); // Grape is not an approved fruit
    let fruit: Fruit = res_Fruit.unwrap(); // just get the fruit from the match statement
    println!("{:?}", fruit); // Will crash because we are trying to unwrap a type string that is not a form of a fruit
    // panicked. Runtime error. Unwrapping a type Grape will not work.
}

fn main(){
    let r: Result<String,String> = open_file("foo.asa");

    match r{
        Ok(program_source) => {
            let parse_tree = parse_source(program_source)
        }
        Err(error) =>{
            // Handle the error
            // - Alert the user an error occured,
            // - Suggest how to fix it
            // - Exit the programp
        }
    }
}

fn open_file(path: &str) -> Result<String,String>{
    Ok("x = 1+1".to_string()); // mocked up file result
}

// Option<T> [a data type error]
// - a special enum type for expressing an operation that can potentially fail, but you don't care why
// - examples: popping a vector; if it's not empty, you'll get Some(T), if it's empty you get None.
// - Some(T)
// - None

fn main(){
    let mut v = Vec<u64>;
    let q: u64 = v.pop().unwrap(); // have to include unwrap because q is an Option<int> instead of an actual int. Source of the error is removed

    let result = match v.pop(){
        Some(v) => v + 4,
        None => 0,
    }

    println!("{:?}", result);
}


