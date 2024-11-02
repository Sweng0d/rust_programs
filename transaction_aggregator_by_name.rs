use std::collections::HashMap;

fn transactions(txs: Vec<(&str, u32)>) -> Result<HashMap<&str, u32>, String> {
    let mut hashes = HashMap::new();

    for (name, amount) in txs {
        *hashes.entry(name).or_insert(0) += amount;
    }

    Ok(hashes)
}

fn main() {
    let txs = vec![
        ("Bruno", 50),
        ("Alice", 20),
        ("Goddim", 99),
        ("Hector", 66),
        ("Achi", 15),
    ];

    match transactions(txs) {
        Ok(output) => println!("The list of transactions are: {:?}", output),
        Err(e) => println!("Error: {}", e),
    }
}
