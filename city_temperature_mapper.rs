use std::collections::HashMap;

fn city_and_temp (city: Vec<&str>, temp: Vec<u16>) -> HashMap<&str, u16> {
    
    let mut country = HashMap::new();

    let mut counter: usize = 0;

    for cities in city {
        country.insert(cities, temp[counter]);
        counter += 1;
    }

    country
}

fn main () {
    let all_cities: Vec<&str> = vec!["Santos", "São Paulo", "Piracicaba", "Rio de Janeiro"];

    let temperatures: Vec<u16> = vec![31, 25, 27, 38];

    let mut hashmap_completed = city_and_temp(all_cities, temperatures);

    for (city, temp) in &hashmap_completed {
        println!("The city is {} and the temp is {}", city, temp);
    }
}
