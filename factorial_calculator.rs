use std::io;

fn main() {
    println!("Choose the number to calculate factorial");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    match input.trim().parse::<u32>() {
        Ok(num) => {
            println!("You choose the number {}", num);

            let mut answer = factorial(num);
            println!("The factorial number is {}", answer);
        }
        Err(_) => {
            println!("Please, insert a valid number");
        }
    }

}

fn factorial (n: u32) -> u32 {

    let mut result = 1;

    for i in 1..= n {
        result *= i;
    }

    result  
}

