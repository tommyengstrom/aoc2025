use std::fs;

fn main() {
    println!("Day 6");

    let parsed = parse_input1(&fs::read_to_string("data/day06.txt").unwrap());
    let result_1 = part1(&parsed);
    println!("Result part 1: {result_1}");

    let result_2 = part2(&parse_input_blocks(&fs::read_to_string("data/day06.txt").unwrap()));
    println!("Result part 2: {result_2}");
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, PartialEq, Clone)]
struct Equation<T: Clone + PartialEq> {
    numbers: Vec<T>,
    operator: Operator,
}



fn parse_input1(s: &str) -> Vec<Equation<u64>> {
    let blocks = parse_input_blocks(s);
    blocks
        .into_iter()
        .map(|e| Equation {
            operator: e.operator,
            numbers: e
                .numbers
                .iter()
                .map(|n| n.trim().parse().unwrap())
                .collect(),
        })
        .collect()
}

fn parse_input_blocks(s: &str) -> Vec<Equation<&str>> {
    let lines: Vec<&str> = s.lines().collect();
    let (operators, number_lines) = lines.split_last().unwrap();
    let lines = number_lines.to_vec();
    BlockParser { operators, lines }.collect()
}

struct BlockParser<'a> {
    operators: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> Iterator for BlockParser<'a> {
    type Item = Equation<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let op = match self.operators.chars().next()? {
            '*' => Operator::Mul,
            '+' => Operator::Add,
            x => panic!("Invalid operator: {x}"),
        };
        let next_operator_index = self.operators[1..].find(|c| c != ' ');
        let length = match next_operator_index {
            Some(_) => self.operators[1..]
                .chars()
                .take_while(|&c| c == ' ')
                .count(),
            None => self.operators.len(),
        };
        let numbers = self.lines.iter().map(|line| &line[..length]).collect();
        let skip = if self.operators.len() > length {
            length + 1
        } else {
            length
        };
        self.lines = self.lines.iter().map(|line| &line[skip..]).collect();
        self.operators = &self.operators[skip..];
        Some(Equation {
            operator: op,
            numbers,
        })
    }
}

fn part1(problems: &[Equation<u64>]) -> u64 {
    problems
        .iter()
        .map(|p| match p.operator {
            Operator::Add => p.numbers.iter().sum(),
            Operator::Mul => p.numbers.iter().fold(0, |a, b| a * b),
        })
        .sum()
}

fn part2(problems: &[Equation<&str>]) -> u64 {
    problems.iter().map(solve_equation).sum()
}

fn solve_equation(eq: &Equation<&str>) -> u64 {
    let mut numbers = vec![];
    for pos in 0..eq.numbers[0].len() {
        let apa: String = eq.numbers.iter().map(|&n| n.chars().nth(pos).unwrap()).collect();
        numbers.push(apa.trim().parse().unwrap());
    }
    match eq.operator {
        Operator::Add => numbers.iter().sum(),
        Operator::Mul => numbers.iter().product(),
    }
}

#[cfg(test)]
mod test06 {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let parsed = parse_input1(input);
        assert_eq!(
            parsed[0],
            Equation {
                numbers: vec![123, 45, 6],
                operator: Operator::Mul
            }
        );
        assert_eq!(
            parsed[1],
            Equation {
                numbers: vec![328, 64, 98],
                operator: Operator::Add
            }
        );
        assert_eq!(
            parsed[2],
            Equation {
                numbers: vec![51, 387, 215],
                operator: Operator::Mul
            }
        );
        assert_eq!(
            parsed[3],
            Equation {
                numbers: vec![64, 23, 314],
                operator: Operator::Add
            }
        );
    }

    #[test]
    fn test_parse_input2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let parsed = parse_input_blocks(input);
        assert_eq!(
            parsed[0],
            Equation {
                numbers: vec!["123", " 45", "  6"],
                operator: Operator::Mul
            }
        );
        assert_eq!(
            parsed[1],
            Equation {
                numbers: vec!["328", "64 ", "98 "],
                operator: Operator::Add
            }
        );
        assert_eq!(
            parsed[2],
            Equation {
                numbers: vec![" 51", "387", "215"],
                operator: Operator::Mul
            }
        );
        assert_eq!(
            parsed[3],
            Equation {
                numbers: vec!["64 ", "23 ", "314"],
                operator: Operator::Add
            }
        );
    }
}
