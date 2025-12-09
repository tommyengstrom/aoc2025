use std::fs;

fn main() {
    let id_ranges = parse_input("data/day02.txt");
    let all_ids: Vec<u64> = id_ranges.into_iter().flat_map(expand_range).collect();
    let sum: u64 = all_ids.iter().flat_map(|&x|extract_repeating_id(x)).sum();

    println!("Day 2!");
    println!("Sum of repeated ids: {sum}");

    let sum2: u64 = all_ids.iter().filter(|&&x| is_repeating_id(x)).sum();
    println!("part 2: {sum2}")


}

fn parse_input(filename: &str) -> Vec<IdRange> {
    fs::read_to_string(filename)
        .expect("Failed to read file {filename}")
        .trim()
        .split(',')
        .map(|values| {
            let (from, to) = values.split_once('-').unwrap();
            let from: u64 = from.parse().unwrap();
            let to: u64 = to.parse().unwrap();
            IdRange { from, to }
        })
        .collect()
}

struct IdRange {
    from: u64,
    to: u64,
}

fn expand_range(r: IdRange) -> Vec<u64> {
    (r.from..r.to + 1).collect()
}

fn extract_repeating_id(id: u64) -> Option<u64> {
    let s = id.to_string();
    let length = s.chars().count();
    let (s1, s2) = s.split_at(length / 2);
    if s1 == s2 { Some(id) } else { None }
}

fn is_repeating_id(id: u64) -> bool {
    let digits: Vec<char> = id.to_string().chars().collect();
    let size = digits.len();
    let mut p: Vec<char> = Vec::with_capacity(digits.len() / 2);

    for d in &digits {
        p.push(*d);
        if p.len() > size / 2 {
            return false;
        }
        if p.repeat(size / p.len()) == digits {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test02 {
    use super::*;

    #[test]
    fn test_extract_repeating_id() {
        assert_eq!(extract_repeating_id(1234), None);
        assert_eq!(extract_repeating_id(1212), Some(1212));
        assert_eq!(extract_repeating_id(12512), None);
        assert_eq!(extract_repeating_id(101), None);
    }
    #[test]
    fn test_is_invalid_id() {
        assert!(!is_repeating_id(1234));
        assert!(is_repeating_id(1212));
        assert!(!is_repeating_id(12512));
        assert!(!is_repeating_id(101));
    }

    fn range(a: u64, b: u64) -> Vec<u64> {
        expand_range(IdRange { from: a, to: b })
            .into_iter()
            .filter(|x| is_repeating_id(*x))
            .collect()
    }

    #[test]
    fn test_invalid_ranges() {
        assert_eq!(range(11, 22), vec![11,22]);
        assert_eq!(range(95, 115), vec![99,111]);
        assert_eq!(range(998, 1012), vec![999, 1010]);
        assert_eq!(range(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(range(222220, 222224), vec![222222]);
        assert_eq!(range(1698522, 1698528), vec![]);
        assert_eq!(range(446443, 446449), vec![446446]);
        assert_eq!(range(38593856, 38593862), vec![38593859]);
        assert_eq!(range(565653, 565659), vec![565656]);
        assert_eq!(range(824824821, 824824827), vec![824824824]);
        assert_eq!(range(2121212118, 2121212124), vec![2121212121]);
    }
}
