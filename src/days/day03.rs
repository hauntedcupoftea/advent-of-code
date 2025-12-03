pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn process_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(
                    x.chars()
                        .filter_map(|x| x.to_digit(10).map(|x| x as i64))
                        .collect(),
                )
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let data = process_input(input);
    let mut sum = 0;
    for line in data {
        let mut max_joltage = 0;
        let mut max_right = line[line.len() - 1];
        for idx in (0..line.len() - 1).rev() {
            let current_value = line[idx];
            let value = current_value * 10 + max_right;
            if value > max_joltage {
                max_joltage = value;
            }
            if current_value > max_right {
                max_right = current_value;
            }
        }
        sum += max_joltage;
    }
    sum
}

fn part2(input: &str) -> i64 {
    let data = process_input(input);
    let mut sum = 0;
    for line in data {
        let mut stack = Vec::with_capacity(line.len());
        let mut drops_allowed = line.len() - 12;
        for num in line {
            while drops_allowed > 0 && !stack.is_empty() && *stack.last().unwrap() < num {
                stack.pop();
                drops_allowed -= 1;
            }
            stack.push(num);
        }
        stack.truncate(12);
        let joltage = stack
            .iter()
            .fold(0_i64, |joltage, digit| joltage * 10 + *digit);
        sum += joltage;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3121910778619);
    }
}
