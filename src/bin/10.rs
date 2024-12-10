use std::{
    collections::{HashMap, HashSet, VecDeque},
    slice::Iter,
};

advent_of_code::solution!(10);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }
}

pub fn next_pos((x, y): (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::North => (x + 0, y - 1),
        Direction::East => (x + 1, y + 0),
        Direction::South => (x + 0, y + 1),
        Direction::West => (x - 1, y + 0),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start_positions) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<(i32, i32), u32>::new(), Vec::new()),
            |(mut hm, mut zeroes), (pos, char)| {
                let digit = char.to_digit(10).unwrap();
                hm.insert(pos, digit);
                if digit == 0 {
                    zeroes.push(pos);
                }

                (hm, zeroes)
            },
        );
    Some(
        start_positions
            .iter()
            .map(|start| {
                let mut to_visit = VecDeque::new();
                to_visit.push_back(*start);
                let mut end_positions = HashSet::new();

                while let Some(current) = to_visit.pop_front() {
                    let digit = map.get(&current).unwrap();

                    for dir in Direction::iterator() {
                        let next = next_pos(current, dir);
                        if let Some(&next_digit) = map.get(&next) {
                            if next_digit == digit + 1 {
                                if next_digit == 9 {
                                    end_positions.insert(next);
                                } else {
                                    to_visit.push_back(next);
                                }
                            }
                        }
                    }
                }
                end_positions.len() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start_positions) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<(i32, i32), u32>::new(), Vec::new()),
            |(mut hm, mut zeroes), (pos, char)| {
                let digit = char.to_digit(10).unwrap();
                hm.insert(pos, digit);
                if digit == 0 {
                    zeroes.push(pos);
                }

                (hm, zeroes)
            },
        );
    Some(
        start_positions
            .iter()
            .map(|start| {
                let mut to_visit = VecDeque::new();
                to_visit.push_back(*start);
                let mut end_positions = Vec::new();

                while let Some(current) = to_visit.pop_front() {
                    let digit = map.get(&current).unwrap();

                    for dir in Direction::iterator() {
                        let next = next_pos(current, dir);
                        if let Some(&next_digit) = map.get(&next) {
                            if next_digit == digit + 1 {
                                if next_digit == 9 {
                                    end_positions.push(next);
                                } else {
                                    to_visit.push_back(next);
                                }
                            }
                        }
                    }
                }
                end_positions.len() as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
