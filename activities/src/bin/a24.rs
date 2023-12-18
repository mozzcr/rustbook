// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|n| n * 3)
        .filter(|n| n > &10)
        .collect();

    println!("{:?}", data)
}
