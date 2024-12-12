use std::{
    collections::{HashMap, HashSet},
    slice::Iter,
};

advent_of_code::solution!(12);

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
    let (map, mut groups) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (
                HashMap::<(i32, i32), char>::new(),
                HashMap::<(i32, i32), HashSet<(i32, i32)>>::new(),
            ),
            |(mut hm, mut groups), (pos, char)| {
                hm.insert(pos, char);
                groups.insert(pos, HashSet::from([pos]));

                (hm, groups)
            },
        );

    for (&k, &v) in map.iter() {
        for dir in Direction::iterator() {
            let in_group = groups.get(&k).unwrap().clone();
            let next = next_pos(k, dir);
            if !in_group.contains(&next) {
                if let Some(&val) = map.get(&next) {
                    if val == v {
                        let next_group = groups.get(&next).unwrap().clone();

                        for &elem in in_group.iter() {
                            groups.entry(elem).and_modify(|g| {
                                g.extend(next_group.clone());
                            });
                        }
                        for &elem in next_group.iter() {
                            groups.entry(elem).and_modify(|g| {
                                g.extend(in_group.clone());
                            });
                        }
                    }
                }
            }
        }
    }
    let (found_groups, _) = groups
        .values()
        .fold((Vec::new(), HashSet::new()), |mut acc, set| {
            if set.is_disjoint(&acc.1) {
                acc.0.push(set);
                acc.1.extend(set);
            }
            acc
        });
    Some(
        found_groups
            .iter()
            .map(|set| {
                let area = set.len() as u32;
                let mut perimeter = 0;
                for &elem in set.iter() {
                    for dir in Direction::iterator() {
                        let next = next_pos(elem, dir);
                        if !set.contains(&next) {
                            perimeter += 1;
                        }
                    }
                }
                area * perimeter
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, mut groups) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (
                HashMap::<(i32, i32), char>::new(),
                HashMap::<(i32, i32), HashSet<(i32, i32)>>::new(),
            ),
            |(mut hm, mut groups), (pos, char)| {
                hm.insert(pos, char);
                groups.insert(pos, HashSet::from([pos]));

                (hm, groups)
            },
        );

    for (&k, &v) in map.iter() {
        for dir in Direction::iterator() {
            let in_group = groups.get(&k).unwrap().clone();
            let next = next_pos(k, dir);
            if !in_group.contains(&next) {
                if let Some(&val) = map.get(&next) {
                    if val == v {
                        let next_group = groups.get(&next).unwrap().clone();

                        for &elem in in_group.iter() {
                            groups.entry(elem).and_modify(|g| {
                                g.extend(next_group.clone());
                            });
                        }
                        for &elem in next_group.iter() {
                            groups.entry(elem).and_modify(|g| {
                                g.extend(in_group.clone());
                            });
                        }
                    }
                }
            }
        }
    }
    let (found_groups, _) = groups
        .values()
        .fold((Vec::new(), HashSet::new()), |mut acc, set| {
            if set.is_disjoint(&acc.1) {
                acc.0.push(set);
                acc.1.extend(set);
            }
            acc
        });
    Some(
        found_groups
            .iter()
            .map(|set| {
                let area = set.len() as u32;
                let mut outline = Vec::new();
                for &elem in set.iter() {
                    for &dir in Direction::iterator() {
                        let next = next_pos(elem, &dir);
                        if !set.contains(&next) {
                            outline.push((dir, next));
                        }
                    }
                }
                let same_x: u32 = outline
                    .iter()
                    .fold(HashMap::new(), |mut acc, elem| {
                        if elem.0 == Direction::East || elem.0 == Direction::West {
                            let x = elem.1 .0;
                            acc.entry((elem.0, x))
                                .or_insert_with(Vec::new)
                                .push(elem.1 .1);
                        }
                        acc
                    })
                    .values()
                    .map(|side| {
                        let mut sorted = side.clone();
                        sorted.sort();

                        let mut parts = 1;
                        for window in sorted.windows(2) {
                            if window[1] - window[0] != 1 {
                                parts += 1;
                            }
                        }
                        parts
                    })
                    .sum();
                let same_y: u32 = outline
                    .iter()
                    .fold(HashMap::new(), |mut acc, elem| {
                        if elem.0 == Direction::North || elem.0 == Direction::South {
                            let y = elem.1 .1;
                            acc.entry((elem.0, y))
                                .or_insert_with(Vec::new)
                                .push(elem.1 .0);
                        }
                        acc
                    })
                    .values()
                    .map(|side| {
                        let mut sorted = side.clone();
                        sorted.sort();

                        let mut parts = 1;
                        for window in sorted.windows(2) {
                            if window[1] - window[0] != 1 {
                                parts += 1;
                            }
                        }
                        parts
                    })
                    .sum();
                let perimeter = same_x + same_y;
                area * perimeter
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
