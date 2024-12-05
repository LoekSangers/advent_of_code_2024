use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
        .fold(
            HashMap::<char, Vec<(i32, i32)>>::new(),
            |mut hm, (x, y, char)| {
                hm.entry(char)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));

                hm
            },
        );
    let (candidate_x, candidate_m, candidate_a, candidate_s) = (
        map.get(&'X')?,
        map.get(&'M')?,
        map.get(&'A')?,
        map.get(&'S')?,
    );

    // Convert the Vecs into HashSets for O(1) lookups
    let candidate_m_set: HashSet<_> = candidate_m.iter().copied().collect();
    let candidate_a_set: HashSet<_> = candidate_a.iter().copied().collect();

    Some(
        candidate_x
            .iter()
            .flat_map(|&(x_x, x_y)| {
                candidate_s
                    .iter()
                    .filter_map(|&(s_x, s_y)| {
                        let x_diff = s_x - x_x;
                        let y_diff = s_y - x_y;
                        if (x_diff == 0 || x_diff == 3 || x_diff == -3)
                            && (y_diff == 0 || y_diff == 3 || y_diff == -3)
                        {
                            let x_step = x_diff / 3;
                            let y_step = y_diff / 3;

                            if candidate_m_set.contains(&(x_x + x_step, x_y + y_step))
                                && candidate_a_set.contains(&(x_x + x_step * 2, x_y + y_step * 2))
                            {
                                Some(1)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, char)| (x, y, char)))
        .fold(
            HashMap::<char, Vec<(i32, i32)>>::new(),
            |mut hm, (x, y, char)| {
                hm.entry(char)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));

                hm
            },
        );
    let candidate_m = map.get(&'M')?;
    let candidate_a = map.get(&'A')?;
    let candidate_s = map.get(&'S')?;

    // Convert the Vecs into HashSets for O(1) lookups
    let candidate_m_set: HashSet<_> = candidate_m.iter().copied().collect();
    let candidate_s_set: HashSet<_> = candidate_s.iter().copied().collect();

    fn check_diagonal(
        candidate_m_set: &HashSet<(i32, i32)>,
        candidate_s_set: &HashSet<(i32, i32)>,
        a_x: i32,
        a_y: i32,
        dx: i32,
        dy: i32,
    ) -> Option<bool> {
        let (lx, ly) = (a_x + dx, a_y + dy);
        let (rx, ry) = (a_x - dx, a_y - dy);

        let lu = if candidate_m_set.contains(&(lx, ly)) {
            Some('M')
        } else if candidate_s_set.contains(&(lx, ly)) {
            Some('S')
        } else {
            None
        };

        let rd = if candidate_m_set.contains(&(rx, ry)) {
            Some('M')
        } else if candidate_s_set.contains(&(rx, ry)) {
            Some('S')
        } else {
            None
        };

        match (lu, rd) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => Some(true),
            _ => None,
        }
    }

    Some(
        candidate_a
            .iter()
            .filter_map(|&(a_x, a_y)| {
                let lu = check_diagonal(&candidate_m_set, &candidate_s_set, a_x, a_y, 1, 1);
                let ld = check_diagonal(&candidate_m_set, &candidate_s_set, a_x, a_y, 1, -1);
                let ru = check_diagonal(&candidate_m_set, &candidate_s_set, a_x, a_y, -1, 1);
                let rd = check_diagonal(&candidate_m_set, &candidate_s_set, a_x, a_y, -1, -1);

                if lu.is_none() || ld.is_none() || ru.is_none() || rd.is_none() {
                    None
                } else {
                    Some(1)
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
