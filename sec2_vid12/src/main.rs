// Functions & Inputs
// Rust Udemy course sec 2, vid 12

fn main() {
    // functions are a discreet subprogram designed to complete a specific task

    // call a very basic function with no inputs or outputs
    basic_fn();
    // call function with inputs
    function_with_inputs("Calabe", 250_000);
    // note that the tutorial uses a different format below, but my compiler won't take it
    // function_with_inputs(name: "Calabe", salary: 250_000);

    // we can also call the function by inputting variables instead
    let full_name: &str = "Calabe";
    let salary_info: i32 = 350_000;
    function_with_inputs(full_name, salary_info);

    // calling func with inputs and an output
    let answer: i32 = function_with_inputs_and_outputs(10, 15);
    println!("The answer for the multiplication problem is {}", answer);

    // calling a function with inputs and multiple outputs
    let (multiplication, addition, subtraction) = function_with_inputs_and_multiple_outputs(10, 15);
    println!("Multiplication : {}, Addition: {}, Subtraction: {}", multiplication, addition, subtraction);
    // using a single tuple to store the output of the same function instead
    let result: (i32, i32, i32) = function_with_inputs_and_multiple_outputs(10, 15);
    println!("Multiplication : {}, Addition: {}, Subtraction: {}", result.0, result.1, result.2);

    // code blocks instead of function
    let full_name: String = {
        let first_name: &str = "Calabe";
        let last_name: &str = "Davis";
        format!("{} {}", first_name, last_name) // note the lack of semicolon on the last statement of a code block
    }; // note that the semicolon is here at the end of the code block
    println!("My name is {}.", full_name);

    // receiving inputs from the user
    let mut n: String = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("invalid input, use float");
    println!("{:?}", n);
}

fn basic_fn() {
    // function body
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("The name is {} and the salary is {}", name, salary);
}

fn function_with_inputs_and_outputs(num1: i32, num2:i32) -> i32 {
    num1 * num2  // note this doesn't get a semicolon after it or the compiler will complain
}

fn function_with_inputs_and_multiple_outputs(num1: i32, num2:i32) -> (i32, i32, i32) {
    // multiple outputs are handled by passing out a tuple
    (num1 * num2, num1 + num2, num1 - num2)  // note this doesn't get a semicolon after it or the compiler will complain
}