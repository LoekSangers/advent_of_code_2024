use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (antenna_groups, max_x, max_y) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<char, Vec<(i32, i32)>>::new(), 0, 0),
            |(mut hm, max_x, max_y), (pos, char)| {
                if char.is_alphanumeric() {
                    hm.entry(char).or_insert_with(Vec::new).push(pos);
                }

                (
                    hm,
                    if max_x > pos.0 { max_x } else { pos.0 },
                    if max_y > pos.1 { max_y } else { pos.1 },
                )
            },
        );

    Some(
        antenna_groups
            .values()
            .flat_map(|antennas| {
                antennas.into_iter().combinations(2).flat_map(|combi| {
                    let a = combi[0];
                    let b = combi[1];

                    let step_x = a.0 - b.0;
                    let step_y = a.1 - b.1;

                    vec![(a.0 + step_x, a.1 + step_y), (b.0 - step_x, b.1 - step_y)]
                })
            })
            .filter(|&(x, y)| {
                return x >= 0 && x <= max_x && y >= 0 && y <= max_y;
            })
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antenna_groups, max_x, max_y) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| ((x as i32, y as i32), char))
        })
        .fold(
            (HashMap::<char, Vec<(i32, i32)>>::new(), 0, 0),
            |(mut hm, max_x, max_y), (pos, char)| {
                if char.is_alphanumeric() {
                    hm.entry(char).or_insert_with(Vec::new).push(pos);
                }

                (
                    hm,
                    if max_x > pos.0 { max_x } else { pos.0 },
                    if max_y > pos.1 { max_y } else { pos.1 },
                )
            },
        );

    Some(
        antenna_groups
            .values()
            .flat_map(|antennas| {
                antennas.into_iter().combinations(2).flat_map(|combi| {
                    let a = *combi[0];
                    let b = *combi[1];

                    let step_x = a.0 - b.0;
                    let step_y = a.1 - b.1;

                    let mut results = Vec::new();
                    results.push(a);

                    let mut dist = 1;
                    while a.0 + dist * step_x >= 0
                        && a.0 + dist * step_x <= max_x
                        && a.1 + dist * step_y >= 0
                        && a.1 + dist * step_y <= max_y
                    {
                        results.push((a.0 + dist * step_x, a.1 + dist * step_y));
                        dist += 1;
                    }
                    dist = -1;
                    while a.0 + dist * step_x >= 0
                        && a.0 + dist * step_x <= max_x
                        && a.1 + dist * step_y >= 0
                        && a.1 + dist * step_y <= max_y
                    {
                        results.push((a.0 + dist * step_x, a.1 + dist * step_y));
                        dist -= 1;
                    }

                    results
                })
            })
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
