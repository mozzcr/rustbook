// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

// use std::collections::HashMap;
use std::{collections::HashMap, io};

fn menu_selection(menu_option: i32, bills: &mut HashMap<String, i32>) -> bool {
    match menu_option {
        1 => {
            println!("Add Bills");
            Some(add_bills(bills));
            true
        }
        2 => {
            println!("View Bills");
            Some(view_bills(bills));
            true
        }
        3 => {
            println!("Remove Bills");
            Some(remove_bills(bills));
            true
        }
        4 => {
            println!("Edit Bills");
            Some(edit_bills(bills));
            true
        }
        5 => {
            println!("Back");
            Some(show_menu());
            true
        }
        _ => false,
    }
}

fn add_bills(bills: &mut HashMap<String, i32>) {
    let name = user_input();
    let amount = amount_input();
    bills.insert(String::from(name), amount);
}

fn view_bills(bills: &HashMap<String, i32>) {
    println!("{:?}", bills)
}

fn remove_bills(bills: &mut HashMap<String, i32>) {
    let name = user_input();
    bills.remove(&name);
}

fn edit_bills(bills: &mut HashMap<String, i32>) {
    let name = user_input();
    let amount = amount_input();
    bills.insert(String::from(name), amount);
}

fn user_input() -> String {
    let mut user_input = String::new();

    println!("Enter a name:");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => println!("Input received"),
        Err(_) => println!("Input not received"),
    }
    let trimmed_input = String::from(user_input.trim());
    trimmed_input
}

fn number_input() -> i32 {
    let mut input = String::new();

    println!("Enter a menu choice:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");
    num
}

fn amount_input() -> i32 {
    let mut input = String::new();

    println!("Enter the bill amount:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");
    num
}

fn show_menu() {
    println!("1 - Add");
    println!("2 - View");
    println!("3 - Remove");
    println!("4 - Edit");
    println!("5 - Back");
}

fn present_menu() -> i32 {
    show_menu();
    let user_choice = number_input();
    user_choice
}

fn main() {
    let mut bills = HashMap::new();

    loop {
        let user_choice = present_menu();
        if user_choice > 5 {
            break;
        }
        let menu_choice = menu_selection(user_choice, &mut bills);
        if menu_choice == false {
            break;
        }
    }
    println!("PROGRAM END")
}
