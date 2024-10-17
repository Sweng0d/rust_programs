use std::io;

fn main() { 
    println!("How much do you want to withdrawn?");
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Falha ao ler linha");

    let mut amount: i32 = input.trim().parse().expect("Please, choose a valid number");

    if amount < 5 {
        println!("The minium amount is 5");
        return;
    }

    let mut bill_100 = 0;
    let mut bill_50 = 0;
    let mut bill_20 = 0;
    let mut bill_10 = 0;
    let mut bill_5 = 0;

    if amount >= 100 {
        bill_100 = amount/100;
        amount = amount % 100;
    }

    if amount >= 50 {
        bill_50 = amount/50;
        amount = amount % 50;
    }

    if amount >= 20 {
        bill_20 = amount/20;
        amount = amount % 20;
    }

    if amount >= 10 {
        bill_10 = amount/10;
        amount = amount % 10;
    }

    if amount >= 5 {
        bill_5 = amount/5;
        amount = amount % 5;
    }

    println!("You will receive {} bills of 100, {} bills of 50, {} bills of 20, {} bills of 10 and {} bills of 5. Your change is {}", bill_100, bill_50, bill_20, bill_10, bill_5, amount);

}
