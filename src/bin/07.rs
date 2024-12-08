advent_of_code::solution!(7);

fn evaluate(target: u64, current_total: u64, to_check: &[u64]) -> bool {
    if to_check.len() == 0 {
        return target == current_total;
    } else {
        return evaluate(target, current_total + to_check[0], &to_check[1..])
            || evaluate(target, current_total * to_check[0], &to_check[1..]);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(':');
                let target = parts.next()?.parse::<u64>().unwrap();
                let numbers = parts
                    .next()?
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                if evaluate(target, numbers[0], &numbers[1..]) {
                    Some(target)
                } else {
                    None
                }
            })
            .sum(),
    )
}
fn evaluate_2(target: u128, current_total: u128, to_check: &[u128]) -> bool {
    if to_check.len() == 0 {
        return target == current_total;
    } else {
        return evaluate_2(target, current_total + to_check[0], &to_check[1..])
            || evaluate_2(target, current_total * to_check[0], &to_check[1..])
            || evaluate_2(
                target,
                current_total as u128 * 10u128.pow(to_check[0].ilog10() + 1) + to_check[0] as u128,
                &to_check[1..],
            );
    }
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(':');
                let target = parts.next()?.parse::<u128>().unwrap();
                let numbers = parts
                    .next()?
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u128>().unwrap())
                    .collect::<Vec<_>>();

                if evaluate_2(target, numbers[0], &numbers[1..]) {
                    Some(target)
                } else {
                    None
                }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
