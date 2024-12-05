use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");

    let greater_than =
        parts
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::<u32, Vec<u32>>::new(), |mut hm, line| {
                if let Some(pages) = line
                    .split('|')
                    .map(|p| p.parse::<u32>().ok())
                    .collect::<Option<Vec<_>>>()
                {
                    hm.entry(pages[1]).or_insert_with(Vec::new).push(pages[0]);
                }

                hm
            });

    Some(
        parts
            .next()
            .unwrap()
            .lines()
            .filter_map(|line| {
                let seq = line
                    .split(',')
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let mut res = Some(seq[seq.len() / 2]);

                for (index, page) in seq.iter().enumerate() {
                    let rest = &seq[(index + 1)..];

                    if let Some(forbidden) = greater_than.get(&page) {
                        if forbidden.into_iter().any(|f| rest.contains(f)) {
                            res = None;
                            break;
                        }
                    };
                }

                res
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");

    let greater_than =
        parts
            .next()?
            .lines()
            .fold(HashMap::<u32, Vec<u32>>::new(), |mut hm, line| {
                if let Some(pages) = line
                    .split('|')
                    .map(|p| p.parse::<u32>().ok())
                    .collect::<Option<Vec<_>>>()
                {
                    hm.entry(pages[1]).or_insert_with(Vec::new).push(pages[0]);
                }

                hm
            });

    Some(
        parts
            .next()?
            .lines()
            .filter_map(|line| {
                let seq = line
                    .split(',')
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let mut res = None;

                for (index, page) in seq.iter().enumerate() {
                    let rest = &seq[(index + 1)..];

                    if let Some(forbidden) = greater_than.get(&page) {
                        if forbidden.into_iter().any(|f| rest.contains(f)) {
                            let mut sorted = seq.clone();

                            sorted.sort_by(|a, b| {
                                // If 'a' must be greater than 'b' according to the `greater_than` map, return a positive value
                                if let Some(forbidden) = greater_than.get(a) {
                                    if forbidden.contains(b) {
                                        return std::cmp::Ordering::Greater;
                                    }
                                }

                                // Otherwise, sort order is already correct and leave it as is.
                                std::cmp::Ordering::Less
                            });

                            res = Some(sorted[sorted.len() / 2]);
                            break;
                        }
                    };
                }

                res
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
