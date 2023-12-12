// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u32,
}

fn can_purchase(customer: Customer) -> Result<String, String> {
    let customer_age = customer.age > 20;

    let can = String::from("Customer can purchase");
    let cannot = String::from("Customer cannot purchase");

    match customer_age {
        true => Ok(can),
        false => Err(cannot),
    }
}

fn main() {
    let customer = Customer { age: 22 };
    let result = can_purchase(customer);
    println!("{:?}", result)
}
