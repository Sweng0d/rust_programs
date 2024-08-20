fn main() {
    let mut x = 0;
    let mut y = 1;
    let fibonacci_max = 10000000;
    let mut fibonacci_sequence = Vec::new();

    // Add the first X and Y to the Array
    fibonacci_sequence.push(x);
    fibonacci_sequence.push(y);

    while y < fibonacci_max {
        let next = x + y;
        x = y;
        y = next;

        // Add the next number to the array
        fibonacci_sequence.push(y);
    }

    // Print all the fibonacci sequence
    println!("Sequência de Fibonacci: {:?}", fibonacci_sequence);

    // Find the element Zth in the array
    let z = 12;
    if z <= fibonacci_sequence.len() {
        println!("O {}-ésimo número na sequência de Fibonacci é: {}", z, fibonacci_sequence[z-1]);
    } else {
        println!("A posição {} está fora da sequência calculada.", z);
    }
}
