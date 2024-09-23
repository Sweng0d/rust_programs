use std::io;

fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();


    println!("Choose your first number");
    io::stdin().read_line(&mut number_one).expect("Cannot read");

    println!("Choose your second number");
    io::stdin().read_line(&mut number_two).expect("Cannot read");

    //all inputs are strings, so we need to convert them to numbers
    let number_one: i32 = number_one.trim().parse().expect("please, insert a valid first number!");
    let number_two: i32 = number_two.trim().parse().expect("please, insert a valid second number!");

    let sum = number_one + number_two;

    println!("The {} + {} is equal to {}", number_one, number_two, sum);
}
