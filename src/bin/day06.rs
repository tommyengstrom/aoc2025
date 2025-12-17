use std::fs;

fn main() {
    println!("Day 6");
    
    let part1: u64 = parse_input(&fs::read_to_string("data/day06.txt").unwrap()).iter().sum();
    println!("Result: {part1}");
}

fn parse_input(s: &str) -> Vec<u64> {
    let lines: Vec<Vec<&str>> = s.lines().map(|l| l.split_whitespace().collect()).collect();
    let [first, second, third, fourth, operator] = lines.as_slice() else {
        panic!("Oh damn, it wasn't 5 rows I guess")
    };

    let mut results: Vec<u64> = vec![];
    for i in 0..first.len() {
        let a: u64 = first[i].parse().unwrap();
        let b: u64 = second[i].parse().unwrap();
        let c: u64 = third[i].parse().unwrap();
        let d: u64 = fourth[i].parse().unwrap();
        results.push(match operator[i] {
            "*" => a * b * c * d,
            "+" => a + b + c + d,
            err => panic!("Unexpected operator {err}"),
        })
    }
    results
}
