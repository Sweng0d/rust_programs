use std::io;

fn main() { 

    println!("Choose 3 numbers and the code will show you the higest");
    
    println!("Lets begin with the first number");    
    let mut choice1 = String::new();

    io::stdin().read_line(&mut choice1).expect("Failed to read the line");
    
    let choice1: i32 = choice1.trim().parse().expect("Please enter a valid number");

    println!("Now the second number");    
    let mut choice2 = String::new();

    io::stdin().read_line(&mut choice2).expect("Failed to read the line");
    
    let choice2: i32 = choice2.trim().parse().expect("Please enter a valid number");

    println!("And now the third one");    
    let mut choice3 = String::new();

    io::stdin().read_line(&mut choice3).expect("Failed to read the line");
    
    let choice3: i32 = choice3.trim().parse().expect("Please enter a valid number");
    
    let highest = if choice1 > choice2 && choice1 > choice3{
        choice1
    } else if choice2 > choice1 && choice2 > choice3{
        choice2
    } else {
        choice3
    };

    println!("The highest number is {}", highest);

}
