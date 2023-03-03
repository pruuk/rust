// Vectors
// For Rust Udemy course, sec 2, video 11


fn main() {
    // vector is a collection of similar elements
    // similar to array, but doesn't have a fixed length
    // all elements in a vector need to be the same type
    let mut number_vec: Vec<i32> = vec![1, 5, 6, 8, 9, 10, 11, 12, 15, 16, 12, 10];

    println!("{}", number_vec[0]);
    println!("{:?}", number_vec); // print the entire vector

    // update an element of the vector
    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_same_elements: Vec<i32> = vec![0;10];

    let mut string_array_1: Vec<&str> = vec!["apple", "tomato", "grape"];
    let string_array_2: Vec<&str> = vec!["Unknown";6];

    println!("{:?}", string_array_1);
    println!("{:?}", string_array_2);

    // update the first element of string_array_1
    string_array_1[0] = "calabe davis";

    let char_vec: Vec<char> = vec!['a', 'p', 'p', 'l', 'e'];

    // creating a vector that is a subset of the number vector on line 9
    let subset_vec: &&[i32] = &&number_vec[0..3];
    println!("The subset of the number vector is {:?}", subset_vec);

    // functions on vectors
    println!("Elements in the array are {}", number_vec.len());

    let check_index: Option<&i32> = number_vec.get(100);
    println!("{:?}", check_index);

    // add element to the end of the vector
    number_vec.push(30);
    number_vec.push(40);
    println!("{:?}", number_vec);

    // remove a value from the vector at a specified index
    number_vec.remove(5);
    println!("{:?}", number_vec);

    // contains function
    println!("The value of 10 exists: {}", number_vec.contains(&10)); // note the & here, this is because we're using the pointer
}
