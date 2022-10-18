use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|bank| bank.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    let mut banks = input.to_owned();
    let mut cycles = 0;
    let mut seen = HashSet::new();

    loop {
        cycles += 1;

        redistribute(&mut banks);

        let cur_state = banks.clone();
        if !seen.insert(cur_state) {
            break;
        }
    }

    cycles
}

fn redistribute(banks: &mut [u32]) {
    let (mut cursor, mut remaining) = {
        let mut max = u32::MIN;
        let mut i = 0;

        for (idx, val) in banks.iter().enumerate() {
            if *val > max {
                max = *val;
                i = idx;
            }
        }

        (i, max)
    };

    banks[cursor] = 0;

    while remaining > 0 {
        cursor += 1;

        if cursor >= banks.len() {
            cursor = 0;
        }

        remaining -= 1;
        banks[cursor] += 1;
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut banks = input.to_owned();
    let mut cycles = 0;
    let mut seen = HashMap::new();

    loop {
        cycles += 1;

        redistribute(&mut banks);

        if let Some(prev) = seen.insert(banks.clone(), cycles) {
            return cycles - prev;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator("0 2 7 0")), 5);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator("0 2 7 0")), 4);
    }
}
