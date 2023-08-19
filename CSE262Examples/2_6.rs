let x = 10;
let q = if x > 5{
    true
}else{
    false
};
println!("{}", q);

let x = 3;
// pattern matching like OCaml, Haskell
let r = match x{
    0..=5 => "Small", // 0 to 5, inclusive
    5..=10 => "Large", // 5 to 10, inclusive
    x => "Other",
};

println!("{:?}", r);

let x = 7;
let r = match x{
    0..=5 => "Small", // 0 to 5, inclusive
    5..=10 => "Large", // 5 to 10, inclusive
    x => "Other",
};

println!("{:?}", r);

let x = 300;
let r = match x{
    0..=5 => "Small", // 0 to 5, inclusive
    5..=10 => "Large", // 5 to 10, inclusive
    x => "Other",
};

println!("{:?}", r);

let x = 300;
let r = match x{
    0..=5 => "Small", // 0 to 5, inclusive
    5..=10 => "Large", // 5 to 10, inclusive
    // This will create an error that does not handle all cases
};

println!("{:?}", r);

let x = 300;
let r = match x{
    0..=5 => "Small", // 0 to 5, inclusive
    5..=10 => "Large", // 5 to 10, inclusive
    x => (), // Have to return something of the same type
};

println!("{:?}", r);

enum Fruit{
    Apple,
    Banana,
    Orange,
    Pear,
}

let x = Fruit::Apple;
let r = match x{
    Fruit::Apple => "Apple",
    Fruit::Banana => "I am a banana",
    _ => "Do not like that fruit",
};

println!("{:?}", r);

enum Color{
    Rgb((u8, u8, u8)),
    Transparent,
    Black,
    White,
    Red,
    Blue,
}

let x = Color::Red;
let r = match x{
    Color::Rgb((255,0,0)) | Color::Red => "Red", // Check if red is used through RGB or color red
    Color::Blue => "Blue",
    _ => "Other Color!",
};

println!("{:?}", r);

let x = Color::Red;
let y = Color::Blue,
let z = Color::Green,
let r = match (x,y,z){
    (Color::Black, Color::White)  | (Color::White, Color::Black)=> "Grayscale",
    (Color::Rgb((255,0,0)) , Color::Blue) | (Color::Red, Color::Blue) => "3D!",
    Color::Rgb((255,0,0)) | Color::Red => "Red", // Check if red is used through RGB or color red
    Color::Blue => "Blue",
    _ => "Other Color!",
};

println!("{:?}", r);