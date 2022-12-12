pub fn part_a(lines: &Vec<String>) -> usize {
    let mut max = 0;
    let mut current = 0;

    for line in lines {
        if line.len() == 0 {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }

    // Account for no blank line at the end
    if current > max {
        max = current;
    }

    max
}

pub fn part_b(lines: &Vec<String>) -> usize {
    let mut max_1 = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;

    let mut current = 0;

    for line in lines {
        if line.len() == 0 {
            if current > max_1 {
                max_3 = max_2;
                max_2 = max_1;
                max_1 = current;
            } else if current > max_2 {
                max_3 = max_2;
                max_2 = current;
            } else if current > max_3 {
                max_3 = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }

    // Account for no blank line at the end
    if current > max_1 {
        max_3 = max_2;
        max_2 = max_1;
        max_1 = current;
    } else if current > max_2 {
        max_3 = max_2;
        max_2 = current;
    } else if current > max_3 {
        max_3 = current;
    }

    max_1 + max_2 + max_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_example() {
        let lines: Vec<String> = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part_a(&lines), 24000)
    }

    #[test]
    fn part_b_example() {
        let lines: Vec<String> = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(part_b(&lines), 45000)
    }
}
