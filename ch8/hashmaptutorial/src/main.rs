use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).expect("can not find team score");
    println!("Blue team's score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
