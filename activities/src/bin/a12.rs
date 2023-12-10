// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Colour {
    White,
    Black,
}

impl Colour {
    fn print(&self) {
        match self {
            Colour::White => println!("colour: White"),
            Colour::Black => println!("colour: Black"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    length: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("height: {:?}", self.height);
        println!("width: {:?}", self.width);
        println!("length: {:?}", self.length);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: u32,
    colour: Colour,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, weight: u32, colour: Colour) -> Self {
        Self {
            dimensions,
            weight,
            colour,
        }
    }

    fn print(&self) {
        self.colour.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}

fn main() {
    let shipping_box = ShippingBox::new(
        Dimensions {
            height: 3.5,
            width: 2.5,
            length: 5.5,
        },
        4,
        Colour::White,
    );
    shipping_box.print();
}
