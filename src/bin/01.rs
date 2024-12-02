use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut team_1, mut team_2) = input.lines().fold(
        (Vec::new(), Vec::new()),
        |(mut list_1, mut list_2), line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            list_1.push(parts[0].parse::<u32>().unwrap());
            list_2.push(parts[1].parse::<u32>().unwrap());

            (list_1, list_2)
        },
    );
    team_1.sort();
    team_2.sort();

    Some(
        team_1
            .iter()
            .zip(team_2.iter())
            .map(|(t_1, t_2)| if t_1 > t_2 { t_1 - t_2 } else { t_2 - t_1 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (team_1, mut team_2) = input.lines().fold(
        (Vec::new(), HashMap::new()),
        |(mut list_1, mut hm), line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            list_1.push(parts[0].parse::<u32>().unwrap());
            let value_ref: &mut u32 = hm.entry(parts[1].parse::<u32>().unwrap()).or_insert(0_u32);

            *value_ref += 1;

            (list_1, hm)
        },
    );

    Some(
        team_1
            .into_iter()
            .map(|id| {
                let count = *team_2.entry(id).or_default();
                // println!("{id} * {count}");
                id * count
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
