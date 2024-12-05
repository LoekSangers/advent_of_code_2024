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
                let pages = line
                    .split('|')
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let value_ref: &mut Vec<u32> = hm.entry(pages[1]).or_insert(Vec::new());

                value_ref.push(pages[0]);

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

                for (index, page) in seq.clone().into_iter().enumerate() {
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
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::<u32, Vec<u32>>::new(), |mut hm, line| {
                let pages = line
                    .split('|')
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();

                let value_ref: &mut Vec<u32> = hm.entry(pages[1]).or_insert(Vec::new());

                value_ref.push(pages[0]);

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

                let mut res = None;

                let mut done = false;
                let mut sorted = seq.clone();

                let l = seq.len();
                while !done {
                    done = true;
                    for (index, page) in sorted.clone().into_iter().enumerate() {
                        let mut cont = true;
                        let rest = &mut sorted.clone()[(index + 1)..];
                        rest.reverse();

                        if let Some(forbidden) = greater_than.get(&page) {
                            for (index_2, r) in rest.into_iter().enumerate() {
                                if forbidden.contains(r) {
                                    sorted.swap(index, l - index_2 - 1);
                                    cont = false;
                                    res = Some(sorted[sorted.len() / 2]);
                                    break;
                                }
                            }
                        }
                        if !cont {
                            done = false;
                            break;
                        }
                    }
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
