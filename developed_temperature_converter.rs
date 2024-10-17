use std::io;

fn main() { 

    loop {
        println!("Choose the conversion you want");
        println!("1 -> Celsius to Fahrenheit");
        println!("2 -> Fahrenheit to Celsius");
        println!("3 -> Exit");
    
        let mut choice = String::new();

        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            println!("Please, choose a valid number");
            continue;
            }
        };

        match choice {
            1 => {
                println!("What is the temperatura in Celsius to convert?");
                let mut celsius_converted = String::new();
    
                io::stdin().read_line(&mut celsius_converted).expect("Failed to read the number");
                let celsius_converted: f64 = match celsius_converted.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please, choose a valid number");
                        continue;
                    }
                };
    
                let temp_converted = celsius_to_fahrenheit(celsius_converted);
                println!("{} Celsius in Fahrenheit is {}", celsius_converted, temp_converted);
    
            }
    
            2 => {
                println!("What is the temperatura in Fahrenheit to convert?");
                let mut fahrenheit_converted = String::new();
    
                io::stdin().read_line(&mut fahrenheit_converted).expect("Failed to read the number");
                let fahrenheit_converted: f64 = match fahrenheit_converted.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please, choose a valid number");
                        continue;
                    }
                };
    
                let temp_converted = fahrenheit_to_celsius(fahrenheit_converted);
                println!("{} Fahrenheit in Celsius is {}", fahrenheit_converted, temp_converted);
    
            }
    
            3 => {
                println!("Exiting...");
                break;
            }
    
            _ => {
                println!("There is not this option.");
            }
        };

    }
    

    

    fn celsius_to_fahrenheit (celsius: f64) -> f64 {
        (celsius * 1.8) + 32.0
    }

    fn fahrenheit_to_celsius (fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) / 1.8
    }

}
