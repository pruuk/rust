// Compound types
// Tuples & Arrays

fn main() {
    // tuples - have a fixed length, cannot grow once declared and defined
    let my_information: (&str, i32) = ("Salary", 40_000);
    println!("{} is equal to {}", my_information.0, my_information.1);
    println!("Another way of printing the whole tuple is {:?}", my_information);

    // de-structuring... aka unpacking a tuple
    let (salary, salary_value) = my_information; // in video, instructor types the individual variables, but i get a compiler error when i try that
    println!("{} is equal to {}", salary, salary_value);

    // can also destructure some but not all of the values in a tuple
    let salary: &str = my_information.0;
    let salary_value: i32 = my_information.1;

    let nested_tuple: (i32, f64, (i32, i32), &str) = (4, 5.0, (3, 2), "Hello");
    let element = nested_tuple.2.0; // pulling first item inside the 3rd element (in the nested tuple)


    // we can also make an empty tuple
    let empty_tuple = ();

    // Arrays!
    // must all have the same type, but can be mutable. we can declare type and length of array
    let mut number_array: [i32; 5] = [4, 5, 6, 8, 9];
    println!("{}", number_array[0]); // printing out a single element of the array
    println!("{:?}", number_array); // printing out the entire array

    // changing/updating an element of an array
    number_array[4] = 5;

    // creating an array of homogeneus values
    let array_with_same_elements: [i32; 10] = [0; 10];
    println!("{:?}", array_with_same_elements);

    let mut string_array_1: [&str; 3] = ["apple", "tomato", "grapes"];
    let string_array_2: [&str; 6] = ["unknown"; 6];
    string_array_1[0] = "calabe davis";  // note that we don't enclose this string in brackets because we're replacing a str element with a str

    let char_array: [char; 5] = ['a', 'p', 'p', 'l', 'e'];

    let mut number_array_1: [i32; 5] = [4, 5, 6, 8, 9];

    // pulling out a subset of the array
    let subset_array: &[i32] = &number_array_1[0..3];
    println!("The subset of values of the array are: {:?}", subset_array);
    // including the 4th value, same thing
    let subset_array_1: &[i32] = &number_array_1[0..=3]; // note these are references to the values in the original array, but do not replace those values!
    println!("The subset of values of the array are: {:?}", subset_array_1);

    // subset_array[0] = 15; this cannot be compiled because we can't change the slice
    
    // length function for arrays
    println!("Elements in the array are {}", number_array_1.len());
    // Number of bytes in the array
    println!("The array is occuptying {} bytes", std::mem::size_of_val(&number_array_1));

    // note you cannot assign to elemnents that are outside the length of the array
    // number_array_1[10] = 5;

    // using get command. will return None because 100 is out of range
    let check_index: Option<&i32> = number_array_1.get(100);  // again, could not include type get(index: 100) like he does in the video
    println!("{:?}", check_index);
}
