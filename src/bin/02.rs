advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|window| {
                        if window[0] > window[1] && window[0] - window[1] < 4 {
                            Some(true)
                        } else if window[1] > window[0] && window[1] - window[0] < 4 {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .reduce(|acc, e| {
                        if let Some(a) = acc {
                            if let Some(b) = e {
                                if a == b {
                                    Some(a)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let list = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let res = list
                    .windows(2)
                    .map(|window| {
                        if window[0] > window[1] && window[0] - window[1] < 4 {
                            Some(true)
                        } else if window[1] > window[0] && window[1] - window[0] < 4 {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .reduce(|acc, e| {
                        if let Some(a) = acc {
                            if let Some(b) = e {
                                if a == b {
                                    Some(a)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .unwrap();

                match res {
                    Some(a) => Some(a),
                    _ => {
                        let mut sub_res: Option<bool> = None;
                        for i in 0..list.len() {
                            let mut tmp_list = list.clone();
                            tmp_list.remove(i);

                            sub_res = tmp_list
                                .windows(2)
                                .map(|window| {
                                    if window[0] > window[1] && window[0] - window[1] < 4 {
                                        Some(true)
                                    } else if window[1] > window[0] && window[1] - window[0] < 4 {
                                        Some(false)
                                    } else {
                                        None
                                    }
                                })
                                .reduce(|acc, e| {
                                    if let Some(a) = acc {
                                        if let Some(b) = e {
                                            if a == b {
                                                Some(a)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .unwrap();

                            if let Some(_) = sub_res {
                                break;
                            }
                        }
                        sub_res
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
