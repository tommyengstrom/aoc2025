use std::fs;

fn main() {
    let id_ranges = parse_input("data/day02.txt");
    let all_ids: Vec<u64> = id_ranges.into_iter().flat_map(expand_range).collect();
    let sum: u64 = all_ids
            .into_iter()
            .flat_map(extract_repeating_id)
            .sum();

    print!("Day 2!");
    print!("Sum of repeated ids: {sum}")
    
}

fn parse_input(filename: &str) -> Vec<IdRange>{
    fs::read_to_string(filename)
        .expect("Failed to read file {filename}")
        .trim()
        .split(',')
        .map(|values| {
            let (from, to) = values.split_once('-').unwrap();
            let from:u64 = from.parse().unwrap();
            let to:u64 = to.parse().unwrap();
            IdRange{from, to}
        })
        .collect()


}

struct IdRange {
    from: u64, 
    to: u64,
}

fn expand_range(r:IdRange) -> Vec<u64> {
    (r.from .. r.to).collect()    
}

fn extract_repeating_id(id: u64) -> Option<u64> {
    let s = id.to_string();
    let length = s.chars().count();
    let (s1, s2) = s.split_at(length / 2);
    if s1 == s2 {
        Some(id)
    }
    else {
        None
    }



}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_repeating_id() {
        assert_eq!(extract_repeating_id(1234), None);
        assert_eq!(extract_repeating_id(1212), Some(12));
        assert_eq!(extract_repeating_id(12512), None);
        assert_eq!(extract_repeating_id(0101), None);
    }
}
