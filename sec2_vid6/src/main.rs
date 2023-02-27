fn main() {
    // this is our first program in this course
    // second line of comment

    /* This is a
    multiple line
    of comment
    */

    println!("Hello from Rust");

    print/*ln*/!("Hello World");

    // print!(10); This line fails to compile due to 10 being mistyped (not a string)
    
    print!("{}", 10);

    // output we substituting into the {} is always added in from left to right (like C)
    println!("The value is {}", 10);

    println!("My first name is {} and my last name is {}", "Calabe", "Davis");

    print!("This is a print command");
    
    print!("This is a print command on the same line");

    println!("");

    println!("This is going to be
    Printed on multiple
    lines");

    println!("\n\n This is going to be printed after one line");

    println!("\t This will have some empty spaces at the beginning");

    println!("This is some text which will be overwritten \r this text will only appear");

    println!("\n\n the first slash n is going to be part of the text.");

    println!("This will print single quote \' and this double quotes \"");

    println!("This will print one back slash \\");

    // positional arguments. THis changes the normal left to right ordering
    println!("\n doing {2} for {1} years and i {0} it", "like", 20, "programming");
    // named arguments. Ordering doesn't matter here either
    println!("{language} is a system programming language which is cool to {activity} in", activity = "code", language = "Rust");
    // using math and arguments
    println!("The sum of 25 + 10 = {}", 25 + 10);
}
