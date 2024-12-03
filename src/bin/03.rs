advent_of_code::solution!(3);
use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}\,\d{1,3})\)").unwrap();
    }

    Some(
        input
            .lines()
            .flat_map(|line| {
                RE.find_iter(line).map(|mul| {
                    let numbers = mul.as_str().replace("mul(", "").replace(")", "");

                    let mut parts = numbers.split(',').map(|s| s.parse::<u32>().unwrap());

                    let mut product = 1_u32;
                    while let Some(n) = parts.next() {
                        product *= n;
                    }

                    product
                })
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}\,\d{1,3})\)").unwrap();
        static ref DN: Regex = Regex::new(r"don\'t\(\).*?do\(\)").unwrap();
        static ref END: Regex = Regex::new(r"don\'t\(\).*?$").unwrap();
    }

    let total_input = input.lines().collect::<Vec<_>>().join("");

    let cleaned_input_1 = DN.replace_all(&total_input, "");
    let cleaned_input_2 = END.replace_all(&cleaned_input_1, "").to_string();

    Some(
        cleaned_input_2
            .lines()
            .flat_map(|line| {
                RE.find_iter(line).map(|mul| {
                    let numbers = mul.as_str().replace("mul(", "").replace(")", "");

                    let mut parts = numbers.split(',').map(|s| s.parse::<u32>().unwrap());

                    let mut product = 1_u32;
                    while let Some(n) = parts.next() {
                        product *= n;
                    }

                    product
                })
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
