use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 85);
    scores.insert(String::from("Red"), 73);
    
    let teams = [String::from("Blue"),String::from("Red")];
    let scores = [85,73];

    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    let blue_score = scores.get(&String::from("Blue"));
    for (k,v) in &scores {
        println!("{}, {}", k, v);
    }
}
