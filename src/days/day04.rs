pub fn run(input: &str) {
    let part1_result = part1(input);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2: {}", part2_result);
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn part1(input: &str) -> i64 {
    let grid = process_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let mut total_kept = 0;

    for r in 0..height {
        for c in 0..width {
            if grid[r as usize][c as usize] != '@' {
                continue;
            }
            let mut adjacent_count = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nr = r + dy;
                    let nc = c + dx;
                    if nr >= 0 && nr < height && nc >= 0 && nc < width {
                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent_count += 1;
                        }
                    }
                }
            }
            if adjacent_count < 4 {
                total_kept += 1;
            }
        }
    }
    total_kept
}

fn part2(input: &str) -> i64 {
    let mut grid = process_input(input);
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let mut total_removed = 0;

    loop {
        let mut total_removed_this_pass = 0;

        for r in 0..height {
            for c in 0..width {
                if grid[r as usize][c as usize] != '@' {
                    continue;
                }
                let mut adjacent_count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let nr = r + dy;
                        let nc = c + dx;
                        if nr >= 0 && nr < height && nc >= 0 && nc < width {
                            if grid[nr as usize][nc as usize] == '@' {
                                adjacent_count += 1;
                            }
                        }
                    }
                }
                if adjacent_count < 4 {
                    total_removed_this_pass += 1;
                    grid[r as usize][c as usize] = 'x';
                }
            }
        }

        if total_removed_this_pass == 0 {
            break;
        }

        total_removed += total_removed_this_pass;
    }
    total_removed
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const TEST_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 43);
    }
}
