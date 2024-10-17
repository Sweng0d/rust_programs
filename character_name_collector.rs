use std::io;

fn main() { 

    println!("Choose 5 names of the characters");

    let mut characters: Vec<String> = Vec::new();
    
    println!("Lets begin with the first character");    
    let mut character1 = String::new();

    io::stdin().read_line(&mut character1).expect("Failed to read the line");
    characters.push(character1.trim().to_string());

    println!("Now the second character");    
    let mut character2 = String::new();

    io::stdin().read_line(&mut character2).expect("Failed to read the line");
    characters.push(character2.trim().to_string());

    println!("Now the third character");    
    let mut character3 = String::new();

    io::stdin().read_line(&mut character3).expect("Failed to read the line");
    characters.push(character3.trim().to_string());

    println!("Now the fourth character");    
    let mut character4 = String::new();

    io::stdin().read_line(&mut character4).expect("Failed to read the line");
    characters.push(character4.trim().to_string());

    println!("Now the fifth character");    
    let mut character5 = String::new();

    io::stdin().read_line(&mut character5).expect("Failed to read the line");
    characters.push(character5.trim().to_string());

    println!
    ("The first character is {},
    The second character is {},
    the third is {},
    the fourth is{} 
    and the fifth and last one is {}", characters[0], characters[1], characters[2], characters[3], characters[4]);


}
