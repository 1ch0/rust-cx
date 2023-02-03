use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 30);

    let teams = vec![String::from("Red"), String::from("Green"), String::from("")];
    let initial_scores = vec![10, 50];
    println!("{:#?}", scores);

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Green"), 20);
    scores.insert(String::from("Green"), 30);
    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
        *map.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", map);
}
