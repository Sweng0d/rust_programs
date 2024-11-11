use std::collections::HashMap;

fn counter(text: &Vec<String>) -> HashMap<String, u32> {

    let mut text_counter = HashMap::new();


    for word in text {
        let count = text_counter.entry(word.clone()).or_insert(0);
        *count +=1;
    }

    text_counter
}

fn main() {

    let input = String::from("Hey, this will will is a text, we need to count how many words there are in this text, and each one of them will be counted and registered on the new hashmap. keep going, each one of you will eventually success.");

    let input_vector: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let word_count = counter(&input_vector);
    println!("{:?}", word_count);

}
