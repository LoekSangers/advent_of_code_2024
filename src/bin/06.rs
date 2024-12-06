use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub enum Tile {
    Open,
    Blocked,
}
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
pub fn next_pos((x, y): (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::North => (x + 0, y - 1),
        Direction::East => (x + 1, y + 0),
        Direction::South => (x + 0, y + 1),
        Direction::West => (x - 1, y + 0),
    }
}
pub fn next_direction(dir: &Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start_position) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<(i32, i32), Tile>::new(), (0, 0)),
            |(mut hm, start), (pos, char)| match char {
                '^' => {
                    hm.insert(pos, Tile::Open);
                    (hm, pos)
                }
                '#' => {
                    hm.insert(pos, Tile::Blocked);
                    return (hm, start);
                }
                '.' => {
                    hm.insert(pos, Tile::Open);
                    return (hm, start);
                }
                _ => (hm, start),
            },
        );
    let mut visited = HashSet::new();

    let mut current_pos = start_position;
    let mut current_direction = Direction::North;
    loop {
        visited.insert(current_pos);
        let next_pos = next_pos(current_pos, &current_direction);

        match map.get(&next_pos) {
            Some(tile) => match tile {
                Tile::Open => {
                    current_pos = next_pos;
                }
                Tile::Blocked => {
                    current_direction = next_direction(&current_direction);
                }
            },
            None => break,
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start_position) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<(i32, i32), Tile>::new(), (0, 0)),
            |(mut hm, start), (pos, char)| match char {
                '^' => {
                    hm.insert(pos, Tile::Open);
                    (hm, pos)
                }
                '#' => {
                    hm.insert(pos, Tile::Blocked);
                    return (hm, start);
                }
                '.' => {
                    hm.insert(pos, Tile::Open);
                    return (hm, start);
                }
                _ => (hm, start),
            },
        );
    let mut visited = HashSet::new();

    let mut current_pos = start_position;
    let mut current_direction = Direction::North;
    loop {
        visited.insert((current_pos, current_direction));

        let next_pos = next_pos(current_pos, &current_direction);

        match map.get(&next_pos) {
            Some(tile) => match tile {
                Tile::Open => {
                    current_pos = next_pos;
                }
                Tile::Blocked => {
                    current_direction = next_direction(&current_direction);
                }
            },
            None => break,
        }
    }

    Some(
        visited
            .iter()
            .filter_map(|&(pos, dir)| {
                let mut tmp_visited = HashSet::new();

                let new_block = next_pos(pos, &dir);
                match map.get(&new_block) {
                    Some(Tile::Open) => {
                        let mut tmp_current_pos = start_position;
                        let mut tmp_current_direction = Direction::North;
                        loop {
                            if !tmp_visited.insert((tmp_current_pos, tmp_current_direction)) {
                                return Some(new_block);
                            };
                            let next_pos = next_pos(tmp_current_pos, &tmp_current_direction);

                            match map.get(&next_pos) {
                                Some(Tile::Open) => {
                                    if next_pos.0 == new_block.0 && next_pos.1 == new_block.1 {
                                        tmp_current_direction =
                                            next_direction(&tmp_current_direction);
                                    } else {
                                        tmp_current_pos = next_pos;
                                    }
                                }
                                Some(Tile::Blocked) => {
                                    tmp_current_direction = next_direction(&tmp_current_direction);
                                }
                                None => break,
                            }
                        }
                        return None;
                    }
                    _ => None,
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
