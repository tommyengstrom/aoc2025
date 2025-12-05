use std::{cmp::max, fs};

fn parse_input(s: &str) -> Vec<Option<Bank>> {
    s.trim().lines().map(parse_bank).collect()
}

fn parse_bank(s: &str) -> Option<Bank> {
    let a: Option<Vec<u32>> = s.chars().map(|c| c.to_digit(10)).collect();
    a.map(|d| Bank { batteries: d })
}

struct Bank {
    batteries: Vec<u32>,
}

fn main() {
    let file_content = fs::read_to_string("data/day03.txt").unwrap();
    let banks: Vec<Option<Bank>> = parse_input(&file_content);
    let banks: Vec<Bank> = banks
        .into_iter()
        .enumerate()
        .map(|(i, b)| match b {
            Some(bank) => bank,
            None => panic!("Row {i} of input is fucked"),
        })
        .collect();
    let total: u32 = banks.iter().map(max_jolts).sum();

    println!("Day 3!");
    println!("Total jolts: {total}");
}

fn max_jolts(bank: &Bank) -> u32 {
    println!("Calculating max");
    bank.batteries
        .iter()
        .rev()
        .fold((None, 0), |(prev_max_digit, max_jolts), d| {
            match prev_max_digit {
                None => (Some(*d), *d),
                Some(p) => (Some(max(*d, p)), max(10 * *d + p, max_jolts))
            }
        })
        .1
}

#[cfg(test)]
mod test03 {

    use super::*;

    #[test]
    fn test_max_jolts() {
        assert_eq!(max_jolts(&parse_bank("987654321111111").unwrap()), 98);
        assert_eq!(max_jolts(&parse_bank("811111111111119").unwrap()), 89);
        assert_eq!(max_jolts(&parse_bank("234234234234278").unwrap()), 78);
        assert_eq!(max_jolts(&parse_bank("818181911112111").unwrap()), 92);
    }
}
