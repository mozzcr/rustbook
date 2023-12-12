// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: u32,
    name: String,
    colour: String,
}

impl Person {
    fn print(&self) {
        println!("{:?}, {:?}", self.name, self.colour)
    }
}

fn main() {
    let person1 = Person {
        age: 20,
        name: String::from("John Smith"),
        colour: String::from("black"),
    };
    let person2 = Person {
        age: 34,
        name: String::from("John Doe"),
        colour: String::from("white"),
    };
    let person3 = Person {
        age: 45,
        name: String::from("John Hancock"),
        colour: String::from("grey"),
    };

    let people = vec![person1, person2, person3];

    for x in people {
        if x.age > 21 {
            x.print()
        }
    }
}
