#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut mem = input.to_owned();
    let mut cursor = 0;
    let mut steps = 0;

    loop {
        steps += 1;

        let next = cursor as i32 + mem[cursor];

        if next < 0 || next as usize >= mem.len() {
            break;
        }

        mem[cursor] += 1;
        cursor = next as usize;
    }

    steps
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut mem = input.to_owned();
    let mut cursor = 0;
    let mut steps = 0;

    loop {
        steps += 1;

        let next = cursor as i32 + mem[cursor];

        if next < 0 || next as usize >= mem.len() {
            break;
        }

        if mem[cursor] >= 3 {
            mem[cursor] -= 1;
        } else {
            mem[cursor] += 1;
        }

        cursor = next as usize;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "0\n\
                 3\n\
                 0\n\
                 1\n\
                 -3"
            )),
            5
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "0\n\
                 3\n\
                 0\n\
                 1\n\
                 -3"
            )),
            10
        );
    }
}
