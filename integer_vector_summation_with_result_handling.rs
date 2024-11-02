fn sum_numbers(numbers: Vec<i32>) -> Result<i32, String> {
    
    let mut count = 0;
    for number in numbers {
        count += number;
    }

    Ok(count)
}

fn main() {
    let numbers = vec![5, 15, 35, 301, 50, 12, 19, 0, 2];

    match sum_numbers(numbers) {
        Ok(sum) => println!("Sum of numbers: {}", sum),
        Err(e) => println!("Erro: {}", e),
    }
}
