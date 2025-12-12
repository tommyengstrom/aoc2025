//use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day 5");
    let file_content = fs::read_to_string("data/day05.txt").unwrap();
    let _ = parse_input(&file_content);
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

#[derive(Debug, PartialEq)]
struct Range {
    from: u64,
    to: u64,
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
}
