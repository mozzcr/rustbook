use chrono::prelude;

// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Colour {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
struct ShirtColour(Colour);
impl ShirtColour {
    fn new(colour: Colour) -> Self {
        Self(colour)
    }
}
#[derive(Debug)]
struct ShoesColour(Colour);
impl ShoesColour {
    fn new(colour: Colour) -> Self {
        Self(colour)
    }
}
#[derive(Debug)]
struct PantsColour(Colour);
impl PantsColour {
    fn new(colour: Colour) -> Self {
        Self(colour)
    }
}

fn print_shirt_colour(colour: ShirtColour) {
    println!("shirt colour = {:?}", colour)
}
fn print_shoes_colour(colour: ShoesColour) {
    println!("shoes colour = {:?}", colour)
}
fn print_pants_colour(colour: PantsColour) {
    println!("pants colour = {:?}", colour)
}

fn main() {
    let shirt_colour = ShirtColour::new(Colour::Gray);
    let shoes_colour = ShoesColour::new(Colour::Blue);
    let pants_colour = PantsColour::new(Colour::White);

    print_shirt_colour(shirt_colour);
    print_shoes_colour(shoes_colour);
    print_pants_colour(pants_colour);
}
