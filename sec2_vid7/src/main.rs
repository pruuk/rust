// Variables and Data Types
// Udemy Rust class, section 2, video 7

fn main() {
    // create variable x
    let x = 15;
    println!("The value of var x = {}", x);

    // x = 60.0; this will create an error, as variables are ummutable in Rust
    // in order to be able to change the value of a variable, it must be explicitly set as mutable
    let mut y: f32 = 12.0;
    println!("The value of var y = {}", y);
    y = 50.0;
    println!("Now the value of var y = {}", y);

    /*
    Rules for naming variables
    - Only letters, digits, and underscores
    - Should begin with letter or underscores
    - Variable names are case sensitive
    */

    /*
    Data types
        SCALAR TYPES:
            - Integers
                - Signed (can be positive or negative "sign")
                    - i8, i16, i32, i64
                - Unsigned
                    - u8, u16, u32, u64 
    */

    println!("The maximum number in i8 is {}", std::i8::MAX);

    println!("The maximum number in u8 is {}", std::u8::MAX);

    /* 
    Floats:
        - f32
        - f64
    */

    let z: f64 = 3.65;

    // let a = x + z This would throw a compiler error because x is a integer and z is a float

    /*
    Booleans:
     */

    let status: bool = false;
    println!("The value of some of our variables are {:?}", (x, y, z, status));

    // booleans are useful
    let not_equals: bool = 18 != 18;
    println!("The value of condition is 18 != 18 is {}", not_equals);

    // char - can be letter, number, emoticon, any special character
    let c1: char = 'a'; // note that single quotes are used for char and double quotes for strings!
    let c2: char = '3'; // note that you cannot perform mathematical operations on a char that is a digit!
    let c3: char = '+';
    let c4: char = '\u{288A}';
    let c5: char = '\"';

    println!("The value of c1 = {}, c2 = {}, c3 = {}, c4= {}, c5 = {}", c1, c2, c3, c4, c5);
}
