use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    for i in 0..10 {
        v.push(i * 2);
    }

    println!("v: {v:#?}");

    let mut s = String::new();

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    for i in &scores {
        println!("{} : {}", i.0, i.1);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
