// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    let ticket1 = Ticket::Backstage(12, String::from("John"));
    let ticket2 = Ticket::Vip(32, String::from("James"));
    let ticket3 = Ticket::Standard(6);

    let tickets = vec![ticket1, ticket2, ticket3];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Holder: {:?}, Price: {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => println!("Holder: {:?}, Price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Price: {:?}", price),
        }
    }
}

enum Ticket {
    Backstage(u32, String),
    Vip(u32, String),
    Standard(u32),
}
