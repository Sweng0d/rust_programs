use std::io;

fn main () {

    println!("Enter integers to sort. Type 'done' to finish");

    let mut numbers: Vec<i32> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() == "done" {
            break;
        }

        match input.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => println!("Please enter a valid integer or type 'done' to finish"),
        }

    }

    if !numbers.is_empty() {
        println!("Original list: {:?}", numbers);

        bubble_sort(&mut numbers);

        println!("Sorted list: {:?}", numbers);
    } else {
        println!("No numbers were entered");
    }
    
}

fn bubble_sort (arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped;

    loop {
        swapped = false;

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        n -= 1;
    }
}
