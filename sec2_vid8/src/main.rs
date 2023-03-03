/*
This is from the Udemy Course "Rust Programming Master Class: From Beginner to Expert"
Shadowing and Constants
*/


fn main() {
    // initialize multiple variables in one statement
    let (first_number, second_number) = (250, 480.22);

    // large number
    let large_number: i32 = 1_000_000;

    // we can create integer overflow errors by using the wrong type
    // ex. let overflow_number: u8 = 256;

    // outputting numbers in octal and hexidecimal formats
    let x: i32 = 255;
    println!("The value of the variable in octal is {:o} and in hexidecimal is {:X} and in binary {:b}", x, x, x);

    // snakecase is the preferred way of naming variables, but the compiler will run if you break this convention
    let Number = 45;

    let n1: i32 = 14;
    let n2: f64 = 15.6;

    // now we'll get a comiler error if we try to add n1 + n2
    // let l3: i32 = n1 + n2;
    let n3: i32 = n1 + n2 as i32; // this works, but note this doesn't actually retype n2 as an integer, it just uses the int value
    println!("The value of n3 is {}", n3);
    // or we can recast n1 as a float
    let n4: f64 = n1 as f64 + n2;
    println!("The value of n4 is {}", n4);

    // Shadowing: declare a variable with the same name as an existing variable
    let s: i32 = 5;
    let s: i32 = 5 * 5;
    println!("The value of s is {}", s); // this outputs s = 25

    // shaodowing a mutable variable
    let mut p: i32 = 5;
    let p: i32 = 5 * 5;
    // p =60; This would throw an error, because p is now immutable despite initially being declared as mutable

    let q: i32 = 32;
    let q: char = 'A'; // we can reassign the value and type as many times as we want

    let mut r: i32 = 65;
    {
        let r: i32 = 60;
        println!("The value of r is {}", r); // scope is limited within brackets. r becomes 60 here

        // however, if we change the value within brackets, it will be permenantly changed
        // r = 60; This would also make the line below outside the brackets output r = 60
    }
    println!("The value of r is {}", r); // r is 65 here, because the shawoing done inside the brackets is out of scope. Scope = main function

    // constants: values that cannot be changed
    // cannot use key word mut with constants
    // types must be annotated on constants
    // naming convention is all UPPERCASE with underscores between words
    const MAX_SALARY: u32 = 100_000;
}
