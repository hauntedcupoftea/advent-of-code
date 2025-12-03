const DIAL_SIZE: i32 = 100;

pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn part1(input: &str) -> i32 {
    let mut current_position: i32 = 50;
    let mut result: i32 = 0;
    for (idx, line) in input.lines().enumerate() {
        let positive: bool = line.chars().nth(0) == Some('R');
        let magnitude: i32 = match line[1..].parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error at line {}: {}", idx, e);
                return 0;
            }
        };
        let dial_operation = if positive { magnitude } else { -magnitude };
        current_position = (current_position + dial_operation + DIAL_SIZE) % DIAL_SIZE;
        if current_position == 0 {
            result += 1
        }
    }
    result
}

fn part2(input: &str) -> i32 {
    let mut current_pos: i32 = 50;
    let mut total_zero_hits = 0;
    for line in input.lines() {
        let is_right = line.starts_with('R');
        let magnitude: i32 = line[1..].parse().unwrap();

        let previous_pos = current_pos;

        if is_right {
            current_pos += magnitude;
            total_zero_hits +=
                current_pos.div_euclid(DIAL_SIZE) - previous_pos.div_euclid(DIAL_SIZE);
        } else {
            current_pos -= magnitude;
            total_zero_hits +=
                (previous_pos - 1).div_euclid(DIAL_SIZE) - (current_pos - 1).div_euclid(DIAL_SIZE);
        }
    }

    total_zero_hits
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
