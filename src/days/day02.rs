pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn process_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .filter_map(|x| {
            let (start, end) = x.split_once('-').expect("Invalid range.");
            Some((
                start.parse().expect("Invalid start"),
                end.parse().expect("Invalid end"),
            ))
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut total_ids: i64 = 0;
    let ranges = process_input(input);
    for (start, end) in ranges {
        for n in start..=end {
            let l = n.checked_ilog10().unwrap_or(0) + 1;
            if l % 2 != 0 {
                continue;
            }
            if n % (10_i64.pow(l / 2) as i64 + 1) == 0 {
                total_ids += n;
            }
        }
    }
    total_ids
}

fn part2(input: &str) -> i64 {
    let mut total_ids: i64 = 0;
    let ranges = process_input(input);
    for (start, end) in ranges {
        for i in start..=end {
            if _is_invalid(i) {
                total_ids += i;
            }
        }
    }
    total_ids
}

fn _is_invalid(n: i64) -> bool {
    let len = n.checked_ilog10().unwrap_or(0) + 1;
    for p_len in 1..=(len / 2) {
        if len % p_len != 0 {
            continue;
        }
        let numerator = 10_i64.pow(len) - 1;
        let denominator = 10_i64.pow(p_len) - 1;
        let mask = numerator / denominator;

        if n % mask == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4174379265);
    }
}
