use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input
        .iter()
        .circular_tuple_windows()
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let (left, right) = input.split_at(input.len() / 2);

    std::iter::zip(left, right)
        .filter_map(|(x, y)| if x == y { Some(x * 2) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator("1122")), 3);
        assert_eq!(solve_part1(&input_generator("1111")), 4);
        assert_eq!(solve_part1(&input_generator("1234")), 0);
        assert_eq!(solve_part1(&input_generator("91212129")), 9);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator("1212")), 6);
        assert_eq!(solve_part2(&input_generator("1221")), 0);
        assert_eq!(solve_part2(&input_generator("123425")), 4);
        assert_eq!(solve_part2(&input_generator("123123")), 12);
        assert_eq!(solve_part2(&input_generator("12131415")), 4);
    }
}
