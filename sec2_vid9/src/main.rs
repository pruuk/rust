// compound data types, Strings

fn main() {
    // two types of strings

    // string slice, fixed length size, cannot be mutated
    let some_string: &str = "Fixed length string"; // this is the default type of string

    // mutable
    let mut growable_string: String = String::from("This string can grow");
    println!("The string is: \"{}\" ", growable_string);

    // operations on the mutable string
    growable_string.push('s'); // in video, instructor uses growable_string.push(ch: 's'); but that won't compile for me
    println!("The string is now: \"{}\" ", growable_string);

    growable_string.pop();
    println!("The string is now: \"{}\" ", growable_string);

    growable_string.push_str(" Which i will use"); // again, including the type like the instructor does creates a compile error for me
    println!("The string is now: \"{}\" ", growable_string);
    // some print functions
    println!("Basic function on Strings,
    is_empty(): {},
    length: {},
    Bytes: {},
    Contains 'use' {}",
    growable_string.is_empty(),
    growable_string.len(),
    growable_string.capacity(),
    growable_string.contains("use"));

    growable_string.push_str("    ");
    let str_len: usize = growable_string.trim().len();
    println!("The length of the growable string with trailing spaces trimmed is {}", str_len);

    let number: i32 = 6;
    let num_str: String = number.to_string();
    println!("Is the number really a string {}", number.to_string() == "6");

    let some_char = 'a';
    let char_str: String = some_char.to_string();


    let my_name: String = "Calabe Davis".to_string();

    // create empty strings
    let empty_string: String = String::new();
    println!("The length of empty string is {}", empty_string.len());

    let s_1: String = "Calabe".to_string();
    let s_2: String = "Davis".to_string();
    let s_3: String = format!("My first name is {} and my last name is {}", s_1, s_2);
    println!("{}", s_3);

    let concat: String = format!("{} {}", s_1, s_2);
    println!("{}", concat);
}
