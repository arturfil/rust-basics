use std::collections::HashMap;



pub fn maps_main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // updating a hash_map
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(66);
    scores.entry(String::from("Blue")).or_insert(66);
    scores.entry(String::from("Orange")).or_insert(66);
    println!("{:?}", scores);

    // iterate over a string and count how many same words it has
    let train = "I hello I jump I world the world is crazy a crazy world jump out of this world I don't know about world views just travel and say hello around the world";
    let mut data = HashMap::new();

    for word in train.split_whitespace() {
        let count = data.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", data);
}

