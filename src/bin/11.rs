use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers = input
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .fold(HashMap::new(), |mut hm, e| {
            *hm.entry(e).or_insert(0_u64) += 1;

            hm
        });
    for _ in 0..25 {
        let mut next_numbers = HashMap::new();
        for (&k, &v) in numbers.iter() {
            if k == 0 {
                *next_numbers.entry(1).or_insert(0_u64) += v;
            } else if k.ilog10() % 2 == 1 {
                let len = k.ilog10() + 1;
                let left = k / 10_u64.pow(len / 2);
                let right = k % 10_u64.pow(len / 2);
                *next_numbers.entry(left).or_insert(0_u64) += v;
                *next_numbers.entry(right).or_insert(0_u64) += v;
            } else {
                *next_numbers.entry(k * 2024).or_insert(0_u64) += v;
            }
        }
        numbers = next_numbers;
    }
    Some(numbers.values().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut numbers = input
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .fold(HashMap::new(), |mut hm, e| {
            *hm.entry(e).or_insert(0_u64) += 1;

            hm
        });
    for _ in 0..75 {
        let mut next_numbers = HashMap::new();
        for (&k, &v) in numbers.iter() {
            if k == 0 {
                *next_numbers.entry(1).or_insert(0_u64) += v;
            } else if k.ilog10() % 2 == 1 {
                let len = k.ilog10() + 1;
                let left = k / 10_u64.pow(len / 2);
                let right = k % 10_u64.pow(len / 2);
                *next_numbers.entry(left).or_insert(0_u64) += v;
                *next_numbers.entry(right).or_insert(0_u64) += v;
            } else {
                *next_numbers.entry(k * 2024).or_insert(0_u64) += v;
            }
        }
        numbers = next_numbers;
    }
    Some(numbers.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
