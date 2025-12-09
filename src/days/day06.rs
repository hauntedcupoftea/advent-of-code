pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn parse_numbers(input: &str) -> Vec<Vec<u128>> {
    let lines: Vec<&str> = input.lines().collect();
    lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u128>().unwrap())
                .collect()
        })
        .collect()
}

fn parse_operators(input: &str) -> Vec<char> {
    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect()
}

fn part1(input: &str) -> u128 {
    let numbers = parse_numbers(input);
    let operators = parse_operators(input);

    let mut total = 0u128;
    let num_problems = operators.len();

    for col in 0..num_problems {
        let mut col_value = numbers[0][col];
        for row in 1..numbers.len() {
            match operators[col] {
                '+' => col_value += numbers[row][col],
                '*' => col_value *= numbers[row][col],
                _ => {}
            }
        }
        total += col_value;
    }

    total
}

fn part2(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().collect();
    for line in &lines {
        println!("{}", line.len());
    }
    let width: usize = lines.iter().map(|l| l.len()).max().unwrap();

    let mut columns: Vec<String> = vec![String::new(); width];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    let mut total = 0u128;
    let mut current_sum = 0u128;
    let mut current_op = '+';

    for col in columns {
        let trimmed = col.trim();

        if trimmed.ends_with('+') || trimmed.ends_with('*') {
            total += current_sum;
            current_op = trimmed.chars().last().unwrap();
            current_sum = match current_op {
                '+' => 0,
                '*' => 1,
                _ => 0,
            };
        }

        let num_str: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
        if !num_str.is_empty() {
            let num = num_str.parse::<u128>().unwrap_or(0);
            current_sum = match current_op {
                '+' => current_sum + num,
                '*' => current_sum * num,
                _ => current_sum,
            };
        }
    }

    total += current_sum;
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3263827);
    }
}
