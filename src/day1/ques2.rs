use std::collections::HashMap;
use std::fs;

pub fn run() {
    let content = fs::read_to_string("src/day1/input1.txt").expect("Error reading from the file");

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in content.lines() {
        let (first, second) = line.split_once("   ").expect("Splitting failed");

        let first_num = first.parse().expect("Conversion failed");
        let second_num = second.parse().expect("Conversion failed");

        match map.get(&second_num) {
            Some(value) => {
                map.insert(second_num, *value + 1);
            }
            None => {
                map.insert(second_num, 1);
            }
        }

        vec1.push(first_num);
        vec2.push(second_num);
    }

    vec1.sort();
    vec2.sort();

    let mut sum = 0;
    for num in vec1 {
        sum += num * map.get(&num).unwrap_or(&0);
    }

    println!("{}", sum);
}