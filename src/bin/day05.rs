use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day 5");
    let file_content = fs::read_to_string("data/day05.txt").unwrap();
    let (ranges, ids) = match parse_input(&file_content) {
        Ok(a) => a,
        Err(err) => {
            panic!("Parse error: {err}");
        }
    };
    let fresh_count = fresh_ingredient_count_explicit(&ranges, &ids);
    println!("Part 1: There are {fresh_count} fresh ingredients");

    let fresh_ingredient_ids = count_fresh_ingredient_ranges(&ranges);
    println!("Part 2: There are {fresh_ingredient_ids} ingredients considered fresh");
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

#[derive(Debug, PartialEq, Clone)]
struct Range {
    from: u64,
    to: u64,
}


fn mk_fresh_ingredient_set(ranges: &[Range]) -> HashSet<u64> {
    ranges
        .iter()
        .map(|r| HashSet::from_iter(r.from..=r.to))
        .reduce(|a, b| &a | &b)
        .unwrap_or_default()
}

fn count_fresh_ingredient_ranges(ranges: &[Range]) -> u64 {
    ranges.iter()
        .fold(Vec::new(), |acc, b| {
            let more_shit = subtract_ranges(&b, &acc);
            [acc, more_shit].concat()
        }).iter().map(|range| range.to - range.from + 1)
        .sum()
}

fn fresh_ingredient_count_explicit(ranges: &Vec<Range>, ids: &[u64]) -> usize {
    fn in_some_range(ranges: &Vec<Range>, id: &u64) -> bool {
        for range in ranges {
            if range.from <= *id && *id <= range.to {
                return true;
            }
        }
        false
    }
    ids.iter().filter(|id| in_some_range(ranges, id)).count()
}

fn subtract_range(a: &Range, b: &Range) -> Vec<Range> {
    if b.from <= a.from && a.to <= b.to {
        vec![]
    } else if a.to < b.from || a.from > b.to {
        vec![a.clone()] // No overlap, no removing
    } else if a.from >= b.from {
        vec![Range {
            from: b.to + 1,
            to: a.to,
        }]
    } else if a.to <= b.to {
        vec![Range {
            from: a.from,
            to: b.from - 1,
        }]
    } else {
        // b is fully inside a
        vec![
            Range {
                from: a.from,
                to: b.from - 1,
            },
            Range {
                from: b.to + 1,
                to: a.to,
            },
        ]
    }
}

fn subtract_ranges(a: &Range, bs: &[Range]) -> Vec<Range> {
    bs.iter().fold(vec![a.clone()], |acc, b| {
        acc.iter().flat_map(|x| subtract_range(&x, &b)).collect()
    })
}

#[cfg(test)]
mod test05 {

    use super::*;

    #[test]
    fn test_subtract_range() {
        assert_eq!(
            subtract_range(&Range { from: 1, to: 10 }, &Range { from: 11, to: 15 }),
            vec![Range { from: 1, to: 10 }]
        );
        assert_eq!(
            subtract_range(&Range { from: 1, to: 10 }, &Range { from: 7, to: 15 }),
            vec![Range { from: 1, to: 6 }]
        );
        assert_eq!(
            subtract_range(&Range { from: 1, to: 10 }, &Range { from: 0, to: 5 }),
            vec![Range { from: 6, to: 10 }]
        );
        assert_eq!(
            subtract_range(&Range { from: 1, to: 10 }, &Range { from: 4, to: 7 }),
            vec![Range { from: 1, to: 3 }, Range { from: 8, to: 10 }]
        );
        assert_eq!(
            subtract_range(&Range { from: 1, to: 10 }, &Range { from: 1, to: 10 }),
            vec![]
        );
    }

    #[test]
    fn test_subtract_ranges() {
        assert_eq!(
            subtract_ranges(
                &Range { from: 1, to: 10 },
                &[Range { from: 0, to: 2 }, Range { from: 6, to: 7 }]
            ),
            vec![Range { from: 3, to: 5 }, Range { from: 8, to: 10 }]
        );
    }

    #[test]
    fn test_parse_input() {
        let input: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, ids) = parse_input(input).unwrap();

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
    fn test_fresh_ingredient_count_explicit() {
        let ranges = vec![
            Range { from: 3, to: 5 },
            Range { from: 10, to: 14 },
            Range { from: 16, to: 20 },
            Range { from: 12, to: 18 },
        ];
        let ids = vec![1, 5, 8, 11, 17, 32];

        let count = fresh_ingredient_count_explicit(&ranges, &ids);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_fresh_ingredient_set() {
        let ranges = vec![
            Range { from: 3, to: 5 },
            Range { from: 10, to: 14 },
            Range { from: 16, to: 20 },
            Range { from: 12, to: 18 },
        ];
        let expected: HashSet<u64> = [3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
            .into_iter()
            .collect();
        let fresh_ids = mk_fresh_ingredient_set(&ranges);
        assert_eq!(fresh_ids, expected);
    }
}
