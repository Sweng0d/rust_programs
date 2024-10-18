use std::io;

fn main() {

    let mut numbers: Vec<i32> = Vec::new();

    println!("Input the numbers. Digit 'done' when you are done.");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("failed to read the line");

        let input = input.trim();

        if input == "done" {
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                numbers.push(num);
            }

            Err(_) => {
                println!("Please, choose a valid number or type 'done'.");
            }
        }
    }

    if !numbers.is_empty() {
        println!("Your list is {:?}", numbers);
        let even_numbers = only_even(&numbers);
        println!("Your list of even numbers is {:?}", even_numbers);
    } else {
        println!("There isn't any number");
    }

} 

fn only_even(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for &i in vec.iter() {
        if i % 2 == 0 {
            new_vec.push(i);
        }
    }

    new_vec
}
