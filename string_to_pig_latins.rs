fn main() {
    let mut string_test = String::from("aimeudeus");
    let converted_word = convert_pig(&string_test);
    println!("From {} to {}", string_test, converted_word);
}

fn convert_pig(word: &str) -> String {
    let mut word_edited: String = word.to_string();

    for (pos, i) in word_edited.chars().enumerate() {
        if i != 'a' && i != 'e' && i != 'i' && i != 'o' && i != 'u' {
            word_edited.push(i);
            word_edited.remove(pos);
            word_edited.push_str("ay");
            break;
        }
    }
    word_edited
}
