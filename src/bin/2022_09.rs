use aoc::{client::get_input, point::Point};
use std::{error::Error, iter, collections::HashSet};

struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn update_tail(&mut self) {
        if isize::abs(self.tail.x - self.head.x) <= 1 && isize::abs(self.tail.y - self.head.y) <= 1 {
            // tail is close enough
            return
        }

        self.tail.y += isize::signum(self.head.y - self.tail.y);
        self.tail.x += isize::signum(self.head.x - self.tail.x);
    }
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: Default::default(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(2022, 9)?;
    println!("solution part 1: {}", part_1(&input)?);
    println!("solution part 2: {}", part_2(&input)?);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| {
            let mut parts = line.split(" ");

            let direction = match parts.next().unwrap() {
                "L" => Point::at(-1, 0),
                "R" => Point::at(1, 0),
                "U" => Point::at(0, 1),
                "D" => Point::at(0, -1),
                _ => unreachable!(),
            };
            let steps = parts.next().unwrap().parse::<usize>().unwrap();

            iter::repeat(direction).take(steps)
        })
        .collect()
}

fn part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let moves = parse_input(input);
    let mut rope = Rope::default();
    let mut visited = HashSet::<Point>::new();
    
    visited.insert(rope.tail);

    for next_move in moves {
        rope.head += next_move;
        rope.update_tail();
        visited.insert(rope.tail);
    }

    Ok(format!("{}", visited.len()))
}

fn part_2(input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), "13")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), "")
    }
}
