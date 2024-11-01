use std::collections::HashMap;

fn receive (string: &str) -> HashMap <String, usize> {
    let string_splited: Vec<&str> = string.split_whitespace().collect();

    let mut words_and_id = HashMap::new();

    for word in string_splited {
        if let Some(count) = words_and_id.get(word) {
            words_and_id.insert(word.to_string(), count + 1);
        } else {
            words_and_id.insert(word.to_string(), 1);
        }
    }
    words_and_id
}

fn main() {

    let mut phrase = receive("the world is yours bro, the fucking entire world world world");
    println!("{:?}", phrase);

}
