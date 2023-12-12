// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furniture = HashMap::new();

    furniture.insert(String::from("chairs"), 5);
    furniture.insert(String::from("beds"), 3);
    furniture.insert(String::from("tables"), 2);
    furniture.insert(String::from("couches"), 0);

    print_furniture(furniture);
}

fn print_furniture(furniture: HashMap<String, i32>) {
    let mut stock = 0;

    for (item, qty) in furniture.iter() {
        stock += qty;
        if qty == &0 {
            println!("furniture: {:?}, out of stock", item)
        } else {
            println!("furniture: {:?}, amount: {:?}", item, qty)
        }
    }
    println!("total stock: {:?}", stock)
}
