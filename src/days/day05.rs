pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn process_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    input
        .split_once("\n\n")
        .map(|(ranges, nums)| {
            (
                ranges
                    .lines()
                    .filter_map(|l| l.split_once('-'))
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .collect(),
                nums.lines().filter_map(|l| l.parse().ok()).collect(),
            )
        })
        .expect("Invalid input format")
}

fn part1(input: &str) -> i64 {
    let (ranges, query) = process_input(input);
    query
        .iter()
        .filter(|&i| ranges.iter().any(|(start, end)| i >= start && i <= end))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let (mut ranges, _) = process_input(input);
    ranges.sort_by_key(|r| r.0);
    let mut merged_count = 0;
    let mut iter = ranges.into_iter();
    if let Some((mut cur_start, mut cur_end)) = iter.next() {
        for (next_start, next_end) in iter {
            if next_start <= cur_end + 1 {
                cur_end = cur_end.max(next_end);
            } else {
                merged_count += cur_end - cur_start + 1;
                cur_start = next_start;
                cur_end = next_end;
            }
        }
        merged_count += cur_end - cur_start + 1;
    }
    merged_count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 14);
    }
}
