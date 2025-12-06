use std::{cmp::max, fs};

fn parse_input(s: &str) -> Vec<Option<Bank>> {
    s.trim().lines().map(parse_bank).collect()
}

fn parse_bank(s: &str) -> Option<Bank> {
    let a: Option<Vec<u64>> = s
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();
    a.map(|d| Bank { batteries: d })
}

struct Bank {
    batteries: Vec<u64>,
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
    let total2: u64 = banks.iter().map(|x| max_jolts(2, x)).sum();
    let total12: u64 = banks.iter().map(|x| max_jolts(12, x)).sum();

    println!("Day 3!");
    println!("Total jolts 2: {total2}");
    println!("Total jolts 12: {total12}");
}


fn max_jolts(size: usize, bank: &Bank) -> u64 {

    let mut result: Vec<u64> = Vec::new(); 

    for &d in bank.batteries.iter() {
        result.push(d);
        if result.len() > size {
            for i in 0..result.len() -1 {
                if result[i] < result[i+1] {
                    result.remove(i);
                    break;
                }
            }
            if result.len() > size {
                result.pop();
            }
        }
    };
    result.iter().fold(0, |acc, d| 10 * acc + d)
}

#[cfg(test)]
mod test03 {

    use super::*;
    fn parsed(s: &str) -> Bank {
        parse_bank(s).unwrap()
    }

    #[test]
    fn test_max_jolts() {
        assert_eq!(max_jolts(2, &parsed("987654321111111")), 98);
        assert_eq!(max_jolts(2, &parsed("811111111111119")), 89);
        assert_eq!(max_jolts(2, &parsed("234234234234278")), 78);
        assert_eq!(max_jolts(2, &parsed("818181911112111")), 92);
    }

    #[test]
    fn test_more_max_jolts() {
        assert_eq!(max_jolts(12, &parsed("987654321111111")), 987654321111);
        assert_eq!(max_jolts(12, &parsed("811111111111119")), 811111111119);
        assert_eq!(max_jolts(12, &parsed("234234234234278")), 434234234278);
        assert_eq!(max_jolts(12, &parsed("818181911112111")), 888911112111);
    }
}
