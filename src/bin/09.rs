advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut file = input
        .lines()
        .flat_map(|line| {
            line.chars().enumerate().flat_map(|(index, n)| {
                let mut res = Vec::new();
                let length = n.to_digit(10).unwrap();
                for _ in 0..length {
                    if index % 2 == 1 {
                        res.push(None);
                    } else {
                        res.push(Some((index / 2) as u64));
                    }
                }
                res
            })
        })
        .collect::<Vec<_>>();

    let mut start_index = 0;
    let mut end_index = file.len() - 1;
    while start_index < end_index {
        if file[start_index] == None {
            while file[end_index] == None {
                end_index -= 1;
            }
            file.swap(start_index, end_index);
            start_index += 1;
            end_index -= 1;
        } else {
            start_index += 1;
        }
    }

    Some(
        file.iter()
            .enumerate()
            .filter_map(|(index, file_id)| match file_id {
                Some(id) => Some(index as u64 * id),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut files, mut free, _) = input.lines().fold((Vec::new(), Vec::new(), 0), |_, line| {
        line.chars()
            .enumerate()
            .fold((Vec::new(), Vec::new(), 0), |mut res, (index, n)| {
                let length = n.to_digit(10).unwrap();

                if index % 2 == 1 {
                    res.1.push((res.2 as u64, length as u64, None::<u64>));
                } else {
                    res.0
                        .push((res.2 as u64, length as u64, Some((index / 2) as u64)));
                }

                (res.0, res.1, res.2 + length)
            })
    });

    let mut file_index = files.len();

    while file_index > 0 {
        file_index -= 1;

        let mut file = files[file_index];

        for elem in (free.clone()).iter().enumerate() {
            if file.1 < elem.1 .1 && file.0 > elem.1 .0 {
                file.0 = elem.1 .0;
                files[file_index] = file;

                free[elem.0] = (elem.1 .0 + file.1, elem.1 .1 - file.1, elem.1 .2);
                break;
            } else if file.1 == elem.1 .1 && file.0 > elem.1 .0 {
                file.0 = elem.1 .0;
                files[file_index] = file;
                free.remove(elem.0);
                break;
            }
        }
    }

    Some(
        files
            .iter()
            .filter_map(|&(index, length, file_id)| match file_id {
                Some(id) => {
                    let mut total = 0;
                    for i in index..(index + length) {
                        total += i * id
                    }
                    Some(total)
                }
                _ => None,
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
