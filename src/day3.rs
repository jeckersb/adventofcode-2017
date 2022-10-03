#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> u32 {
    input.parse().unwrap()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &u32) -> u32 {
    if *input == 1 {
        return 0;
    }

    let mut ring_max = 0;
    let ring = (1..)
        .step_by(2)
        .find(|n| {
            ring_max = n * n;
            ring_max >= *input
        })
        .unwrap()
        / 2;

    let side_length = ring * 2 + 1;
    let distance = (ring_max - *input) % (side_length - 1);
    let midpoint = side_length / 2;
    let offset = midpoint.abs_diff(distance);

    ring + offset
}

const NEIGHBORS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const MOTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[aoc(day3, part2)]
pub fn solve_part2(input: &u32) -> u32 {
    let mut points = std::collections::HashMap::new();
    let mut cursor = (0, 0);

    points.insert(cursor, 1);

    let mut steps = 1;
    let mut incr_steps = false;
    let mut motions = MOTIONS.iter().cycle();

    loop {
        let motion = motions.next().unwrap();

        for _ in 0..steps {
            cursor = (cursor.0 + motion.0, cursor.1 + motion.1);

            let val = NEIGHBORS
                .iter()
                .map(|neigh| {
                    points
                        .get(&(cursor.0 + neigh.0, cursor.1 + neigh.1))
                        .unwrap_or(&0)
                })
                .sum();

            if val > *input {
                return val;
            }

            points.insert(cursor, val);
        }

        if incr_steps {
            steps += 1;
            incr_steps = false;
        } else {
            incr_steps = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator("1")), 0);
        assert_eq!(solve_part1(&input_generator("12")), 3);
        assert_eq!(solve_part1(&input_generator("23")), 2);
        assert_eq!(solve_part1(&input_generator("1024")), 31);

        assert_eq!(solve_part1(&input_generator("7")), 2);
        assert_eq!(solve_part1(&input_generator("9")), 2);
        assert_eq!(solve_part1(&input_generator("25")), 4);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator("1")), 2);
        assert_eq!(solve_part2(&input_generator("12")), 23);
        assert_eq!(solve_part2(&input_generator("23")), 25);
        assert_eq!(solve_part2(&input_generator("800")), 806);
    }
}
