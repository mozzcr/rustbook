// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Orange,
    Apple,
    Banana,
}

struct Drink {
    flavour: Flavour,
    fluid_ounces: i32,
}

fn print_drink(drink: Drink) {
    match drink.flavour {
        Flavour::Orange => println!("flavour: orange"),
        Flavour::Apple => println!("flavour: apple"),
        Flavour::Banana => println!("flavour: banana"),
    }
    println!("fluid_ounces: {:?}", drink.fluid_ounces)
}

fn main() {
    let orange_drink = Drink {
        flavour: Flavour::Orange,
        fluid_ounces: 12,
    };

    print_drink(orange_drink)
}
