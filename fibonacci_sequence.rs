fn main() {
    let mut x = 0;
    let mut y = 1;
    let fibonacci_max = 1000000;
    
    
    println!("{}", x);
    
    while y < fibonacci_max {
        println!("{}",y);
        
        let next = x + y;
        x = y;
        y = next;
    }
    
}


