// For Udemy's Rust course
// Section 2, assignment 3

// Questions for this assignment
// 1. Modify the program below by adding suitable functions so that it compiles correctly
/*
fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

}

fn add_3 (number: i32) -> i32 {
    number + 3
}

fn add_4 (number: i32) -> i32 {
    number + 4
}

fn times (num1: i32, num2: i32) -> i32 {
    num1 * num2
}
*/

// 2. Rewrite the main function in a way so that there is no variable in it and it performs the same job as this program. Your program should make calls to both the functions in this program.

/*
fn double(x: i32) -> i32 {

    x * 2

}

fn triple(x: i32) -> i32 {

    x * 3

}

fn main() {

    let (x, y) = (triple(double(5)), triple(triple(double(5))));

    println!("Answer: {}", y);

}

// 3. Write a function which will accept a tuple called point representing the x-axis and y-axis coordinates of a point. 
//The function will compute the distance of the point from the origin and will return the computed distance.

// The template of the function is given below

fn print_distance(point: (f32, f32)) -> f64{ 

// compute the distance from x, y to the origin (0,0)
(( point.0 as f64 - 0.0).powf(2.0) + ( point.1 as f64 - 0.0).powf(2.0)).sqrt()

}
/*
Inside the function, first destructure the tuple into (x,y). This provides a better readability instead of using point.0 and point.1. 
Next compute the distance from the original using the formula of √(x − 0)2 + (y − 0)2.  you may consider using the following two functions 1). 
 x.powf(2.0) for computing the square of the number x. 2). x.sqrt() which will compute the square root of a number x.



Test the program with the following main program
*/


fn main() {

println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));

// Note: we need to enclose the inputs to the function in double paranthesis, i.e., print_distance((5.0,4.0)).

// This is becuase a single paranthesis will mean two inputs of 5.0 and 4.0 and since the function has one

// input which is a single tuple therefore the compiler will complain.  
}
*/
/*
// 4. Complete the following code given below by filling in the code corresponding to the  comments



 fn main(){

    // Step 1: Write a print statement for asking user to input the width of a rectangle

    println!("Enter the width of a rectangle: ");



    // Step 2: Write code for taking the input from the user of type u32 and store it in the variable of width

    // receiving inputs from the user
    let mut width: String = String::new();
    std::io::stdin()
    .read_line(&mut width)
    .expect("failed to read input.");

    let width: f64 = width.trim().parse().expect("invalid input, use float");


    // Step 3: Write a print statement for asking the user to input the length of a rectangle

    println!("Enter the length of a rectangle: ");



    // Step 4: Write code for taking the input from the user of type u32 and store it in the variable of length

    let mut length: String = String::new();
    std::io::stdin()
    .read_line(&mut length)
    .expect("failed to read input.");

    let length: f64 = length.trim().parse().expect("invalid input, use float");  



    let resultant_area: f64 = {

        // Step 5: call a function area() inside here with inputs of width and length which will return the area
        area(length, width)

    };

    // Step 6: write code to print the resultant_area variable to the terminal
    println!("The area of the rectangle is {}", resultant_area);

 }



 fn area(length:f64, width:f64) -> f64 {

    // Step 7: write the definition of the area here which is length * width and return the result
    length * width

 }
*/

/*
5. In this exercise question, we will be writting some code that will help us cook an awesome pizza from a given cooking book.

You are required to write four functions corresponding to some tasks. Some of the tasks may be related to each other also.

Function 1.

Write a function called expected_minutes_in_oven that will return a value which indicates how many minutes the pizza should be in the oven.
According to the cooking book, the expected oven time in minutes is 40: Below is the skeleton of the function indicating what is expected from the function to return. 

fn expected_minutes_in_oven() -> i16 {
    40
}
// Returns: 40



Function 2

Write a function called remaining_minutes_in_oven that takes the actual minutes the pizza has been in the oven as an input
parameter and returns how many minutes the pizza  still has to remain in the oven, based on the expected oven time in minutes from the previous task.

fn remaining_minutes_in_oven(minutes_pizza_in_oven: i32) --> i32 {
    expected_minutes_in_oven() - minutes_pizza_in_oven
}

// Returns: 10 becuase 40- 30 = 10.

Function 3

Write a function called preparation_time_in_minutes that takes the number of toppings you added to the pizza as a parameter and
returns how many minutes you spent preparing the pizza, assuming each layer takes you 2 minutes to prepare.

fn preparation_time_in_minutes(toppings: i32) -> i32 {
    toppings * 2
}

// Returns: 4



Function 4

Write a function called elapsed_time_in_minutes that takes two input parameters: the first parameter is the number of toppings you added to the pizza, 
and the second parameter is the number of minutes the pizza has been in the oven.
The function should return how many minutes you've worked on cooking the pizza, 
which is the sum of the preparation time in minutes, and the time in minutes the pizza  has spent in the oven at the moment.

fn elapsed_time_in_minutes(toppings: i32, minutes_pizza_in_oven: i32) --> i32 {
    minutes_pizza_in_oven + preparation_time_in_minutes(toppings)
}

// Returns: 26
*/
/*
6. Consider the program below. Modify the definition of the quadruple function below by calling the double function twice
(this means that the quadruple function should only make call to the double function and it should call it twice).
The quadruple function should return 4 times the number that has been provided to it as an input
*/
fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32)-> i32 {
    // call the double function twice
    double(double(x))
}

fn main() {

    println!("For 1: the expected value is 4 while the output is {}", quadruple(1));

    println!("For 2: the expected value is 8 while the output is {}", quadruple(2));

    println!("For 3: the expected value is 12 while the output is {}", quadruple(3));

    println!("For 4: the expected value is 16 while the output is {}", quadruple(4));

}
