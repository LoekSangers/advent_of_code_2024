use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(index_y, line)| {
            line.chars()
                .enumerate()
                .map(move |(index_x, char)| (index_x, index_y, char))
        })
        .fold(
            HashMap::<char, Vec<(i32, i32)>>::new(),
            |mut hm, (x, y, char)| {
                let value_ref: &mut Vec<(i32, i32)> = hm.entry(char).or_insert(Vec::new());

                value_ref.push((x as i32, y as i32));

                hm
            },
        );
    let candidate_x = map.get(&'X').unwrap();
    let candidate_m = map.get(&'M').unwrap();
    let candidate_a = map.get(&'A').unwrap();
    let candidate_s = map.get(&'S').unwrap();

    Some(
        candidate_x
            .iter()
            .flat_map(|(x_x, x_y)| {
                candidate_s.iter().filter_map(move |(s_x, s_y)| {
                    let x_diff = (s_x - x_x).abs();
                    let y_diff = (s_y - x_y).abs();
                    if (x_diff == 0 || x_diff == 3) && (y_diff == 0 || y_diff == 3) {
                        Some(((x_x, x_y), (s_x, s_y)))
                    } else {
                        None
                    }
                })
            })
            .filter_map(|((x_x, x_y), (s_x, s_y))| {
                let x_step = (s_x - x_x) / 3;
                let y_step = (s_y - x_y) / 3;

                if candidate_m.contains(&(x_x + x_step, x_y + y_step))
                    && candidate_a.contains(&(x_x + x_step * 2, x_y + y_step * 2))
                {
                    Some(true)
                } else {
                    None
                }
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(index_y, line)| {
            line.chars()
                .enumerate()
                .map(move |(index_x, char)| (index_x, index_y, char))
        })
        .fold(
            HashMap::<char, Vec<(i32, i32)>>::new(),
            |mut hm, (x, y, char)| {
                let value_ref: &mut Vec<(i32, i32)> = hm.entry(char).or_insert(Vec::new());

                value_ref.push((x as i32, y as i32));

                hm
            },
        );
    let candidate_m = map.get(&'M').unwrap();
    let candidate_a = map.get(&'A').unwrap();
    let candidate_s = map.get(&'S').unwrap();

    Some(
        candidate_a
            .iter()
            .filter_map(|(a_x, a_y)| {
                let lu: Option<char> = if candidate_m.contains(&(a_x - 1, a_y - 1)) {
                    Some('M')
                } else if candidate_s.contains(&(a_x - 1, a_y - 1)) {
                    Some('S')
                } else {
                    None
                };
                let ld: Option<char> = if candidate_m.contains(&(a_x - 1, a_y + 1)) {
                    Some('M')
                } else if candidate_s.contains(&(a_x - 1, a_y + 1)) {
                    Some('S')
                } else {
                    None
                };
                let ru: Option<char> = if candidate_m.contains(&(a_x + 1, a_y - 1)) {
                    Some('M')
                } else if candidate_s.contains(&(a_x + 1, a_y - 1)) {
                    Some('S')
                } else {
                    None
                };
                let rd: Option<char> = if candidate_m.contains(&(a_x + 1, a_y + 1)) {
                    Some('M')
                } else if candidate_s.contains(&(a_x + 1, a_y + 1)) {
                    Some('S')
                } else {
                    None
                };
                if lu == None || ld == None || ru == None || rd == None {
                    None
                } else {
                    let diag_1 = match lu {
                        Some('M') => {
                            if rd == Some('S') {
                                true
                            } else {
                                false
                            }
                        }
                        Some('S') => {
                            if rd == Some('M') {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };
                    let diag_2 = match ru {
                        Some('M') => {
                            if ld == Some('S') {
                                true
                            } else {
                                false
                            }
                        }
                        Some('S') => {
                            if ld == Some('M') {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    if diag_1 && diag_2 {
                        Some(true)
                    } else {
                        None
                    }
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
