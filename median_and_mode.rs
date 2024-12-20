use std::collections::HashMap;

fn main() {
    let list: Vec<i32> = vec![2, 4, 6, 7, 10, 12, 14, 2, 3, 4, 6, 4, 6, 4, 4, 8, 10];

    let median = calculate_median(&list);
    println!("The median is {}", median);

    let mode = calculate_mode(&list);
    println!("The mode is {}", mode);

}

fn calculate_median(list: &Vec<i32>) -> i32 {
    
    let median;

    if list.len() % 2 == 0 {
        median = (list[list.len()/2 - 1] + list[1 + list.len()/2 - 1])/ 2;
    } else {
        median = list[1 + list.len()/2 - 1];
    }

    median
}

fn calculate_mode(list: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
     
    for i in list {
        let count = hash.entry(i).or_insert(0);
        *count += 1;
    }

    let mut higher = 0;
    let mut the_i: &i32 = &0;
    for (i, j) in &hash {

        if *j > higher {
            higher = *j;
            the_i = i;
        }
    }
    *the_i
}


