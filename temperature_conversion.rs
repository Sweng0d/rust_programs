Convert temperatures between Fahrenheit and Celsius.

fn main() {
    let temp_celsius = 120;
    let temp_cel_to_fah = celsius_to_fahrenheit(temp_celsius);
    println!("The new temperature in Fahrenheit is {}", temp_cel_to_fah);

    let temp_fahrenheit = -60;
    let temp_fah_to_cel = fahrenheit_to_celsius(temp_fahrenheit);
    println!("The new temperature in Celsius is {}", temp_fah_to_cel);
}

fn celsius_to_fahrenheit(x:i32) -> i32 {
    x * 9 / 5 + 32
}

fn fahrenheit_to_celsius(x:i32) -> i32 {
    (x - 32) * 5 / 9
}




