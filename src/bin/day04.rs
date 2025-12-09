use std::fs;

fn main() {
    println!("Day 4");
    let file_content = fs::read_to_string("data/day04.txt").unwrap();
    let grid = parse_input(&file_content);
    println!("Movable rolls {0}", movable_rolls(&grid));
    println!("Total movable rolls {0}", total_movable_rolls(&grid));
}

#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T: Clone + Default> Grid<T> {
    fn get(&self, x: i32, y: i32) -> T {
        if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.data.len() / self.width) as i32
        {
            return T::default();
        };
        self.data[(y as usize) * self.width + (x as usize)].clone()
    }

    fn windows(&self) -> impl Iterator<Item = Grid<T>> + '_ {
        let height: usize = self.data.len() / self.width;

        (0..height).flat_map(move |y| (0..self.width).map(move |x| self.subgrid(x, y)))
    }

    fn subgrid(&self, x: usize, y: usize) -> Grid<T> {
        Grid {
            data: [-1, 0, 1]
                .iter()
                .flat_map(move |dy| {
                    [-1, 0, 1]
                        .iter()
                        .map(move |dx| self.get(x as i32 + dx, y as i32 + dy))
                })
                .collect(),
            width: 3,
        }
    }
}

fn parse_input(s: &str) -> Grid<bool> {
    s.lines()
        .map(|line| Grid {
            data: line
                .chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    bad => panic!("Bad character: {bad}"),
                })
                .collect(),
            width: line.len(),
        })
        .reduce(|a, b| {
            if a.width != b.width {
                panic!("Width mismatch: {0}, {1}", a.width, b.width)
            };
            Grid {
                data: [a.data, b.data].concat(),
                width: a.width,
            }
        })
        .expect("No lines :(")
}

fn is_movable(grid: &Grid<bool>) -> bool {
    if grid.width != 3 || grid.data.len() != 9 {
        panic!("Called with wrong sized grid!")
    };
    grid.get(1, 1) && grid.data.iter().map(|x| *x as usize).sum::<usize>() <= 4
}

fn movable_rolls(grid: &Grid<bool>) -> usize {
    grid.windows().map(|x| is_movable(&x) as usize).sum()
}

fn remove_movable_rolls(grid: &Grid<bool>) -> Grid<bool> {
    Grid {
        data: grid
            .windows()
            .map(|x| if is_movable(&x) { false } else { x.data[4] })
            .collect(),
        width: grid.width,
    }
}

fn count_rolls(grid: &Grid<bool>) -> usize {
    grid.data.iter().map(|&x| x as usize).sum()
}

fn total_movable_rolls(grid: &Grid<bool>) -> usize {
    let count_before = count_rolls(grid);
    let remove_all = std::iter::successors(Some(grid.clone()), |g| {
        let next_g = remove_movable_rolls(g);

        if g.data == next_g.data {
            None
        } else {
            Some(next_g)
        }
    })
    .last()
    .unwrap();
    let count_after = count_rolls(&remove_all);
    count_before - count_after
}

#[cfg(test)]
mod test04 {

    use super::*;

    #[test]
    fn test_is_movable() {
        let grid1: Grid<bool> = Grid {
            data: (0..9).map(|_| true).collect(),
            width: 3,
        };
        let grid2: Grid<bool> = Grid {
            data: [
                [true, true, true],
                [false, true, false],
                [false, false, false],
            ]
            .concat(),
            width: 3,
        };

        let grid3: Grid<bool> = Grid {
            data: [
                [true, true, true],
                [false, true, false],
                [true, true, false],
            ]
            .concat(),
            width: 3,
        };

        assert!(!is_movable(&grid1));
        assert!(is_movable(&grid2));
        assert!(!is_movable(&grid3));
    }

    #[test]
    fn test_windows() {
        let grid: Grid<u64> = Grid {
            data: (1..10).collect(),
            width: 3,
        };
        assert_eq!(
            grid.windows().collect::<Vec<Grid<u64>>>()[0].data,
            [[0, 0, 0], [0, 1, 2], [0, 4, 5]].as_flattened(),
        );

        assert_eq!(grid.windows().map(|_| 1_usize).sum::<usize>(), 9);
    }
    #[test]
    fn test_movable_rolls() {
        let raw_grid = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let grid = parse_input(raw_grid);
        assert_eq!(movable_rolls(&grid), 13);
    }

    #[test]
    fn test_total_movable_rolls() {
        let raw_grid = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let grid = parse_input(raw_grid);
        assert_eq!(total_movable_rolls(&grid), 43);
    }
}
