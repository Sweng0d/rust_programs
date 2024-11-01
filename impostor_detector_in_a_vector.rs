fn find_element (vec: Vec<&str>, target: &str) -> &'static str {
    for item in vec {
        if item == target {
            panic!("The impostor unfortunately is here");
        }
    }
    "The impostor is not here"

}

fn main () {
    let mut group = vec!["Bruno", "Henrique", "Thiago", "Matheus", "Pedro", "Alan"];

    let answer = find_element(group, "Impostor");
    println!("{}", answer);
}
