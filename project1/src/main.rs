use std::io; // this is a module that comes with the standard library for input/output

fn main() {
    // this is how you print to the console
    println!("Hello, world!");

    // Variables
    let mut x = 5; // mut is used to make a variable mutable so it an be reassigned
    println!("The value of x is: {}", x);

    // this is a different scope within the main function
    {
        let x = x - 3;
        println!("The value of x is: {}", x);
    }

    x = x + 7; // changing the value of x because it is mutable
    println!("The value of x is: {}", x);

    let  x = "some string"; // this works because we are defining a new variable with the "let" keyword
    println!("The value of x is: {}", x);

    const SECONDS_IN_MINUTE: u32 = 60; // this is a constant variable (it is immutable) and it is a good practice to use uppercase for the variable name
    println!("{}", SECONDS_IN_MINUTE);

    // Data Types
    let i: i32 = -132; // this is an example of type annotation for an integer (i8, i16, i32, i64, i128)
    let u: u32 = 132; // this is an example of type annotation for an unsigned integer (u8, u16, u32, u64, u128) that does not allow negative numbers
    println!("{}", i);
    println!("{}", u);

    let floating_point: f32 = 3.14; // this is an example of type annotation for a floating point number (f32 = single precision, f64 = double precision)
    println!("{}", floating_point);
    let true_or_false: bool = true; // this is an example of type annotation for a boolean
    println!("{}", true_or_false);
    let character: char = 'a'; // this is an example of type annotation for a character
    println!("{}", character);

    // Tuples
    let tup: (i32, bool, char) = (1, true, 'j');
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // Arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[2] = 7; // access an element in an array and reassign it
    println!("{}", arr[2]);

    // User Input
    println!("Type something and press Enter: ");
    let mut input = String::new(); // this is how you create a new string
    io::stdin().read_line(&mut input).expect("Failed to read line"); // this is how you read console input from the user with error handling
    println!("You typed: {}", input);

    // Integer Overflows & Mixing Types
    // let a: u8 = 12; // 0 - 255
    // let b: i8 = 10; // -128 - 127
    // let c = a + b; // this will not work because you cannot mix types

    let c = 255.0f64; // this is another way to specify a type
    let d = 10.0_f64; // this is another way to specify a type

    let e = c / d;
    println!("{}", e);

    let f = c * d;
    println!("{}", f);

    let g = c % d; // this is the remainder of the division (modulus)
    println!("{}", g);

    let h = 127_000 as i64; // explicit type conversion (casting)
    println!("{}", h);

    println!("Type a number and press Enter: ");

    let mut input_value = String::new(); 
    io::stdin().read_line(&mut input_value).expect("Failed to read line"); 

    let int_input: i64 = input_value.trim().parse().unwrap(); // this is how you convert a string to an integer

    println!("{}", int_input + 2);

    // Conditions/Control Flow
    if int_input > 10 {
        println!("The number is greater than 10");
    } else if int_input < 10 {
        println!("The number is less than 10");
    } else {
        println!("The number is equal to 10");
    }

    let food = "apple";

    if food == "apple" {
        println!("The food is an apple");
    } else {
        println!("The food is not an apple");
    }

    test_one(); // calling a function outside of main

    add_numbers(6, 8); // calling a function with parameters

    loops(); // calling a function with loops

    let number = {
        let x = 3;
        x + 1
    };
    println!("Statement: {}", number);

    let result = add_numbers_expression(9, 7);
    println!("The sum is: {}", result);

    let sum = add_numbers_result(9, 7);
    println!("The sum is: {}", sum);

    let answer = add_numbers_condition(9, 7);
    println!("{}", answer);
}

// Functions
fn test_one() {
    println!("Test function has been called");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

fn add_numbers_expression(x: i32, y: i32) -> i32 {
    x + y
}

fn add_numbers_result(x: i32, y: i32) -> i32 {
    return x + y; // this is how you return a value from a function (semicolon is optional)
}

fn add_numbers_condition(x: i32, y: i32) -> i32 {
    let answer = x + y;
    if answer > 10 {
        return answer - 10;
    }
    answer
}

// Loops
fn loops() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("The counter is: {}", counter);

        if counter == 10 {
            break;
        }
    }

    let mut counter = 0;

    while counter < 10 {
        counter += 1;
        println!("The counter is: {}", counter);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}
