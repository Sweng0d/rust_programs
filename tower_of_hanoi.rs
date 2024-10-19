use std::io;

fn main () {
    //tower_of_hanoi_problem

    println!("Enter the number of disks: ");
    let mut disks = String::new();
    io::stdin().read_line(&mut disks).expect("Failed to read input");

    let n: u32 = disks.trim().parse().expect("Please enter a valid number");

    hanoi(n, 'A', 'C', 'B');
    
}

fn hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("Move disk 1 from {} to {}", from, to);
    } else {
        hanoi(n-1, from, aux, to);
        println!("Move disk {} from {} to {}", n, from, to);
        hanoi(n - 1, aux, to, from);
    }

}
