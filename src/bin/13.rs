advent_of_code::solution!(13);

struct ClawMachine {
    a_x: i128,
    a_y: i128,
    b_x: i128,
    b_y: i128,
    prize_x: i128,
    prize_y: i128,
}

impl ClawMachine {
    pub fn new(input: &str) -> Self {
        let numbers = input
            .lines()
            .map(|line| {
                line.split(&['+', '=', ','])
                    .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
                    .map(|n| n.parse::<i128>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            a_x: numbers[0][0],
            a_y: numbers[0][1],
            b_x: numbers[1][0],
            b_y: numbers[1][1],
            prize_x: numbers[2][0],
            prize_y: numbers[2][1],
        }
    }

    pub fn solve(&self) -> Option<u32> {
        let b = (self.prize_y * self.a_x - self.prize_x * self.a_y)
            / (self.b_y * self.a_x - self.b_x * self.a_y);
        let a = (self.prize_x - b * self.b_x) / self.a_x;

        if a * self.a_x + b * self.b_x == self.prize_x
            && a * self.a_y + b * self.b_y == self.prize_y
            && a >= 0
            && b >= 0
        {
            Some(3 * a as u32 + b as u32)
        } else {
            None
        }
    }

    pub fn solve_2(&self) -> Option<u128> {
        let b = (self.prize_y * self.a_x - self.prize_x * self.a_y)
            / (self.b_y * self.a_x - self.b_x * self.a_y);
        let a = (self.prize_x - b * self.b_x) / self.a_x;

        if a * self.a_x + b * self.b_x == self.prize_x
            && a * self.a_y + b * self.b_y == self.prize_y
            && a >= 0
            && b >= 0
        {
            Some(3 * a as u128 + b as u128)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .filter_map(|line| ClawMachine::new(line).solve())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(
        input
            .split("\n\n")
            .filter_map(|line| {
                let mut cm = ClawMachine::new(line);
                cm.prize_x += 10000000000000;
                cm.prize_y += 10000000000000;
                cm.solve_2()
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
