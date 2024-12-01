use std::fs;

pub fn run() {
    // Handle the Result using `.expect()` or propagate the error with `?`
    let content = fs::read_to_string("src/day1/input1.txt").expect("Failed to read file");

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in content.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            vec1.push(first.to_string().parse().expect("Not a valid number!"));
            vec2.push(second.to_string().parse().expect("Not a valid number"));
        }
    }

    // println!("Vec1: {:?}", vec1);
    // println!("Vec2: {:?}", vec2);

    vec1.sort();
    vec2.sort();

    let mut sum = 0;

    for i in 0..vec1.len() {
        sum += (vec1[i] - vec2[i]).abs();
    }

    println!("{}", sum);
}
