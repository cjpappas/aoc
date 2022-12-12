pub fn part_a(lines: &Vec<String>) -> usize {
    let mut score = 0;

    for line in lines {
        let round: Vec<&str> = line.split(" ").collect();

        let opponent = *round.get(0).unwrap();
        let player = *round.get(1).unwrap();

        match opponent {
            "A" => match player {
                "X" => {
                    score += 4;
                }
                "Y" => {
                    score += 8;
                }
                "Z" => {
                    score += 3;
                }
                _ => {}
            },
            "B" => match player {
                "X" => {
                    score += 1;
                }
                "Y" => {
                    score += 5;
                }
                "Z" => {
                    score += 9;
                }
                _ => {}
            },
            "C" => match player {
                "X" => {
                    score += 7;
                }
                "Y" => {
                    score += 2;
                }
                "Z" => {
                    score += 6;
                }
                _ => {}
            },
            _ => {}
        }
    }

    score
}

pub fn part_b(lines: &Vec<String>) -> usize {
    let mut score = 0;

    for line in lines {
        let round: Vec<&str> = line.split(" ").collect();

        let opponent = *round.get(0).unwrap();
        let player = *round.get(1).unwrap();

        match opponent {
            "A" => match player {
                "X" => {
                    score += 3;
                }
                "Y" => {
                    score += 4;
                }
                "Z" => {
                    score += 8;
                }
                _ => {}
            },
            "B" => match player {
                "X" => {
                    score += 1;
                }
                "Y" => {
                    score += 5;
                }
                "Z" => {
                    score += 9;
                }
                _ => {}
            },
            "C" => match player {
                "X" => {
                    score += 2;
                }
                "Y" => {
                    score += 6;
                }
                "Z" => {
                    score += 7;
                }
                _ => {}
            },
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_example() {
        let lines: Vec<String> = ["A Y", "B X", "C Z"].map(String::from).to_vec();

        assert_eq!(part_a(&lines), 15);
    }

    #[test]
    fn part_b_example() {
        let lines: Vec<String> = ["A Y", "B X", "C Z"].map(String::from).to_vec();

        assert_eq!(part_b(&lines), 12);
    }
}
