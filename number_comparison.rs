use std::fmt::Display;

fn compare <T: PartialOrd + Display> (x: T, y: T) -> T {
    if x > y {
        println!("{} is bigger than {}", x, y);
        x
    } else if y < x{
        println!("{} is lower than {}", x, y);
        y
    } else {
        println!("{} is equal to {}", x, y);
        x
    }
}

fn main() {
    let mut x:i32 = 25;
    let mut y:i32 = 25;

    compare(x, y);
    
}
