use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();

    for line in input.lines() {
        let mut nums = Vec::new();

        for n in line.split_whitespace() {
            nums.push(n.parse().unwrap());
        }

        rows.push(nums)
    }

    rows
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|row| match row.iter().minmax() {
            itertools::MinMaxResult::MinMax(min, max) => max - min,
            itertools::MinMaxResult::OneElement(_) => 0,
            itertools::MinMaxResult::NoElements => panic!("unexpected empty row"),
        })
        .sum()
}

fn div_pair(nums: &[u32]) -> u32 {
    for (i, v1) in nums.iter().enumerate() {
        for v2 in &nums[i + 1..] {
            if v1 % v2 == 0 {
                return v1 / v2;
            }

            if v2 % v1 == 0 {
                return v2 / v1;
            }
        }
    }

    panic!("no divisible pair");
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|row| div_pair(row)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "5 1 9 5\n\
		 7 5 3\n\
		 2 4 6 8"
            )),
            18
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "5 9 2 8\n\
		 9 4 7 3\n\
		 3 8 6 5"
            )),
            9
        );
    }
}
