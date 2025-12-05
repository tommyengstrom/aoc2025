use std::fs;

fn main() {
    let rotations: Vec<Rotation> = parse_rotations("data/day01.txt");

    let zeros = count_zeros(&rotations);
    println!("Numer of zeros: {zeros}");
}
// Plan of attack
// 1. Create data type `Rotation` for representing rotations
// 2. Implement rotation logic
// 3. Parse file as array of `Rotation`
// 4. Calculate the number of times the dial points to 0


fn parse_rotations(filename: &str) -> Vec<Rotation> {
   fs::read_to_string(filename)
        .expect("Failed to read file {filename}")
        .lines()
        .map(|line| {
            let (direction,steps) = line.split_at(1);
            let n: u64 = steps.parse().expect("Invalid number");
            match direction {
                "L" => Rotation::Left(n),
                "R" => Rotation::Right(n),
                _ => panic!("Invalid rotation: {direction}"),
            }
        }
        )
        .collect()
}

#[derive(Clone)]
enum Rotation {
    Left(u64),
    Right(u64),
}

fn count_zeros(rs: &Vec<Rotation>) -> u64 {
    let mut zeros: u64 = 0;
    let mut pos = 50;
    for r in rs {
        pos = rotate(pos, r);
        if pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

/// Rotates the dial between 0-99.
fn rotate(pos: u64, rotation: &Rotation) -> u64 {
    let r = match rotation {
        Rotation::Left(i) if *i > pos => 100 + pos - (i % 100),
        Rotation::Left(i) => pos - i,
        Rotation::Right(i) => pos + i,
    };
    r % 100
}

#[cfg(test)]
mod test01 {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate(0, &Rotation::Right(10)), 10);
        assert_eq!(rotate(95, &Rotation::Right(10)), 5);
        assert_eq!(rotate(495, &Rotation::Right(10)), 5);
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate(99, &Rotation::Left(77)), 22);
        assert_eq!(rotate(0, &Rotation::Left(1)), 99);
        assert_eq!(rotate(0, &Rotation::Left(101)), 99);
    }

    #[test]
    fn test_count_zeros() {
        assert_eq!(count_zeros(&[Rotation::Right(1)].to_vec()), 0);
        assert_eq!(count_zeros(&[Rotation::Right(50)].to_vec()), 1);
        assert_eq!(
            count_zeros(&[Rotation::Left(50), Rotation::Left(75), Rotation::Right(75)].to_vec()),
            2
        );
    }
}
