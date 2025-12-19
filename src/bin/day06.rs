use std::fs;

fn main() {
    println!("Day 6");

    let parsed = parse_input(&fs::read_to_string("data/day06.txt").unwrap());
    let part1 = sum_of_results(&parsed);
    println!("Result part 1: {part1}");
    let transformed: Vec<Equation> = parsed.iter().map(transform_problem).collect();
    let part2: u64 = sum_of_results(&transformed);
    println!("Result part 2: {part2}");
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug,PartialEq, Clone)]
struct Equation {
    numbers: Vec<u64>,
    operator: Operator,
}

fn parse_input(s: &str) -> Vec<Equation> {
    let lines: Vec<Vec<&str>> = s.lines().map(|l| l.split_whitespace().collect()).collect();
    let (operators, number_lines) = lines.split_last().unwrap();

    number_lines
        .iter()
        .for_each(|l| assert_eq!(l.len(), operators.len(), "Row length mismatch"));

    let mut results: Vec<Equation> = vec![];
    for i in 0..operators.len() {
        let mut numbers: Vec<u64> = vec![];
        for l in number_lines {
            match l[i].parse() {
                Ok(n) => numbers.push(n),
                Err(err) => panic!("Unable to parse: {err}"),
            }
        }
        let op = match operators[i] {
            "*" => Operator::Mul,
            "+" => Operator::Add,
            err => panic!("Unexpected operator {err}"),
        };
        results.push(Equation {
            numbers,
            operator: op,
        });
    }
    results
}

fn sum_of_results(problems: &[Equation]) -> u64 {
    problems.iter()
            .map(|p| match p.operator {
                Operator::Add => p.numbers.iter().sum(),
                Operator::Mul => p.numbers.iter().fold(0, |a,b| a * b),
        }).sum()
}

fn transform_problem(problem: &Equation) -> Equation {
    problem.clone()
}


#[cfg(test)]
mod test06 {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 328  51 64 \n 45 64  387 23 \n 6 98  215 314\n*   +   *   +  ";
        let parsed = parse_input(&input);
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
                numbers: vec![51,387,215],
                operator: Operator::Mul
            }
        );
        assert_eq!(
            parsed[3],
            Equation {
                numbers: vec![64,23,314],
                operator: Operator::Add
            }
        );
    }

    #[test]
    fn test_transform_problem() {
        let p1 =Equation { numbers: vec![123, 45, 6], operator: Operator::Mul };
        let p1_after =Equation { numbers: vec![356, 24, 1], operator: Operator::Mul };

        let p2 = Equation { numbers: vec![328, 64, 98], operator: Operator::Add };
        let p2_after = Equation { numbers: vec![8, 248, 369], operator: Operator::Add };

        let p3 = Equation { numbers: vec![51,387,215], operator: Operator::Mul };
        let p3_after = Equation { numbers: vec![175,581,32], operator: Operator::Mul };

        let p4 = Equation { numbers: vec![64,23,314], operator: Operator::Add };
        let p4_after = Equation { numbers: vec![4,431,623], operator: Operator::Add };

        assert_eq!(transform_problem(&p1), p1_after);
        assert_eq!(transform_problem(&p2), p2_after);
        assert_eq!(transform_problem(&p3), p3_after);
        assert_eq!(transform_problem(&p4), p4_after);

    }
}
