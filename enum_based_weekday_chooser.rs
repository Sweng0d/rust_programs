use std::io;

enum DaysOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main () {
    println!("Choose a number for the day of the week");
    println!("1 - Monday");
    println!("2 - Tuesday");
    println!("3 - Wednesday");
    println!("4 - Thursday");
    println!("5 - Friday");
    println!("6 - Saturday");
    println!("7 - Sunday");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the input");
    let choice: u32 = input.trim().parse().expect("Please, insert a valid number");

    let day = match choice {
        1 => DaysOfWeek::Monday,
        2 => DaysOfWeek::Tuesday,
        3 => DaysOfWeek::Wednesday,
        4 => DaysOfWeek::Thursday,
        5 => DaysOfWeek::Friday,
        6 => DaysOfWeek::Saturday,
        7 => DaysOfWeek::Sunday,
        _ => {
            println!("Invalid choice. Please choose a number between 1 and 7.");
            return
        }
    };

    match day {
        DaysOfWeek::Monday => println!("Today is Monday."),
        DaysOfWeek::Tuesday => println!("Today is Tuesday."),
        DaysOfWeek::Wednesday => println!("Today is Wednesday."),
        DaysOfWeek::Thursday => println!("Today is Thursday."),
        DaysOfWeek::Friday => println!("Today is Friday."),
        DaysOfWeek::Saturday => println!("Today is Saturday."),
        DaysOfWeek::Sunday => println!("Today is Sunday."),
    }
}
