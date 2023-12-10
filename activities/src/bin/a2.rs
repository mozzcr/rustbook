// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display_result()
}

fn add_two_numbers(a: i8, b: i8) -> i8 {
    a + b
}

fn display_result() {
    println!("{:?}", add_two_numbers(1, 3));
}
