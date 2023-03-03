/* 
Assignment 2 in section 2 of the Udemy Rust Course

1.
Define two tuples calls p1 and p2 which will represents points and will have two values, one for the x-axis and one for y-axis.
Write a program to display the absolute difference of the values of x-axis and the y-axis on the terminal.

Note: use the abs() function for determining the absolute value of the difference. 
The compiler may complain when using this function with the message of "ambiguous numeric type". 
In this case make sure that you write infront of all the values "as f64" to get rid of the possible issues.  
Example (-3.5 as f64).abs() will result in a value of 3.5

fn main() {
    let p1 = (4.0, 3.0);
    let p2 = (5.0, 4.5); 
    println!("The difference ofbetween the two points on x-axis is {} and on y-axis is {}", 
    (p1.0 as f64 - p2.0 as f64).abs(), (p1.1 as f64 - p2.1 as f64).abs());
}

2.
In this question, we will implement the same problem as disscussed in Question 1 but wtih the help of arrays. 

Declare two arrays of names p1 and p2 both having length of 2 and type f64.
These two arrays will represent two points along with their coordinate values for the x-axis and for y-axis.
Write a simple program to display the absolute difference of the x-axis and the y-axis coordinates for the two points onto the terminal.

Note: use the abs() function for determining the absolute value of the difference.
The compiler wont have any issues in this case so you dont need to explicitly tell the compiler that the types are f64 for all the values.
The compiler in this case is able to determine the type that is being passed to the absolution function.

fn main() {
    let p1:[f64;2] = [4.0, 3.0];
    let p2:[f64;2] = [5.0, 4.5]; 
    println!("The difference between the two points on x-axis is {} and on y-axis is {}", 
    (p1[0] - p2[0]).abs(), (p1[1] -p2[1]).abs());
}

3.
The distance between two points p1 with coordinates (x1, y1) and p2 with coordinates (x2,y2) is computed using the formula

sqrt ((x1 - x2)^2 + (y1-y2)^2 )   where ^ = is used to indicate the exponent.
Write a program which will first declare two points p1 and p2 using tuples and will Initialize the tuple p1 from the values of (4.0, 3.0)
and the tuple p2 from the values of (5.0, 4.5). Next, write a statement for determining their distance and then display the result to the terminal terminal. 

Note 1: Use the function of sqrt() which is used to compute the square root of the input number. The syntax is number.sqrt().
Note 2: Use the function powf(n) which will compute the nth power of the input number. Example: (3.5 as f64).powf(2.0)  is going to computer the square of 3.5.

fn main() {
    let p1 = (4.0, 3.0);
    let p2= (5.0, 4.5);
    println!("The distance between the two points is {}", 
    (( p1.0 as f64 - p2.0 as f64).powf(2.0) + 
    ( p1.1 as f64 - p2.1 as f64).powf(2.0)).sqrt());
}

4.
Change the  program below by writting the correct data types insted of the DATA_TYPES_PLEASE to make this program compile.

fn main() {

    let a: DATA_TYPES_PLEASE = -15;

    let b: DATA_TYPES_PLEASE = 170;

    let my_name: FIXME = "Michael";

    println!("My name is {}, and the answer is {}", my_name, a * b);

}
*/
fn main() {
    let a: i16 = -15;
    let b: i16  = 170;
    let my_name: &str = "Calabe";
    println!("My name is {}, and the answer is {}", my_name, a * b);
}



