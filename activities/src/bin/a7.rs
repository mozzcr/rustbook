// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Blue,
    Red,
    Green,
    White,
    Black,
}

fn print_colour(colour: Colour) {
    match colour {
        Colour::Blue => println!("blue"),
        Colour::Red => println!("red"),
        Colour::Green => println!("green"),
        Colour::White => println!("white"),
        Colour::Black => println!("black"),
    }
}

fn main() {
    print_colour(Colour::Blue)
}
