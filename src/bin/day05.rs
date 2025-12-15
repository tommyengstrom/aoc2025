use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day 5");
    let file_content = fs::read_to_string("data/day05.txt").unwrap();
    let (ranges, ids) = match parse_input(&file_content) {
           Ok(a) => a,
           Err(err) =>{
            panic!("Parse error: {err}");
        },
        
    };
    let fresh_count = fresh_ingredient_count_explicit(&ranges, &ids);
    println!("There are {fresh_count} fresh ingredients")
}

fn parse_input(s: &str) -> Result<(Vec<Range>, Vec<u64>), &str> {
    let (ranges, ids): (&str, &str) = s.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|l| -> Result<Range, &str> {
            let (a, b) = l.split_once('-').ok_or("Missing - in range")?;
            Ok(Range {
                from: a.parse().map_err(|_| "invalid range start")?,
                to: b.parse().map_err(|_| "invalid range end")?,
            })
        })
        .collect::<Result<Vec<_>, &str>>()?;
    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();
    Ok((ranges, ids))
}

fn mk_fresh_ingredient_set(ranges: &Vec<Range>) -> HashSet<u64> {
    ranges.iter().flat_map(|r| r.from..=r.to).collect()
}

#[derive(Debug, PartialEq)]
struct Range {
    from: u64,
    to: u64,
}

fn fresh_ingredient_count_set(ranges: &Vec<Range>, ids: &Vec<u64>) -> usize {
    let fresh_ingredient_set = mk_fresh_ingredient_set(&ranges);
    ids.iter().filter(|id| fresh_ingredient_set.contains(id)).count()
}

fn fresh_ingredient_count_explicit(ranges: &Vec<Range>, ids: &Vec<u64>) -> usize {
    fn in_some_range(ranges: &Vec<Range>, id: &u64) -> bool {
        for range in ranges {
           if range.from <= *id && *id <= range.to {
            return true
            }
        }
        false
    }
    ids.iter().filter(|id| in_some_range(&ranges, id)).count()
}

#[cfg(test)]
mod test05 {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, ids) = parse_input(&input).unwrap();

        assert_eq!(
            ranges,
            vec![
                Range { from: 3, to: 5 },
                Range { from: 10, to: 14 },
                Range { from: 16, to: 20 },
                Range { from: 12, to: 18 }
            ]
        );

        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_fresh_ingredient_count_set() {
        let ranges = vec![
                Range { from: 3, to: 5 },
                Range { from: 10, to: 14 },
                Range { from: 16, to: 20 },
                Range { from: 12, to: 18 }];
        let ids =vec![1, 5, 8, 11, 17, 32];
        
        let count = fresh_ingredient_count_set(&ranges, &ids);
        assert_eq!(count, 3);
        
    }
    #[test]
    fn test_fresh_ingredient_count_explicit() {
        let ranges = vec![
                Range { from: 3, to: 5 },
                Range { from: 10, to: 14 },
                Range { from: 16, to: 20 },
                Range { from: 12, to: 18 }];
        let ids =vec![1, 5, 8, 11, 17, 32];
        
        let count = fresh_ingredient_count_explicit(&ranges, &ids);
        assert_eq!(count, 3);
        
    }
}
