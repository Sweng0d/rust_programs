fn main() {
    let mut Vector: Vec<String> = Vec::new();

    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            Vector.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
        Vector.push("Fizz".to_string());
        } else if i % 5 == 0 {
        Vector.push("Buzz".to_string());
        } else {
        Vector.push(i.to_string());
        }
    };

    println!("The vector is {:?}", Vector);
}
