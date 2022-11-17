use itertools::Itertools;

struct Puzzle {
    list: Vec<u8>,
    cursor: usize,
    skip: usize,
    lengths: Vec<u8>,
}

fn reverse(head: &mut [u8], tail: Option<&mut [u8]>) {
    if let Some(tail) = tail {
        // TODO: optimize
        let mut nums = head.iter().chain(tail.iter()).copied().collect::<Vec<_>>();
        nums.reverse();
        for (h, n) in head.iter_mut().zip(nums.iter()) {
            *h = *n;
        }
        for (t, n) in tail.iter_mut().zip(nums.iter().skip(head.len())) {
            *t = *n;
        }
    } else {
        head.reverse();
    }
}

impl Puzzle {
    fn new(last: u8, lengths: &str) -> Self {
        let list = (0..=last).collect();
        let lengths: Vec<u8> = lengths.split(',').filter_map(|n| n.parse().ok()).collect();

        Puzzle {
            list,
            lengths,
            cursor: 0,
            skip: 0,
        }
    }

    fn new2(last: u8, lengths: &str) -> Self {
        let list = (0..=last).collect();
        let mut lengths: Vec<u8> = lengths.as_bytes().to_vec();
        lengths.extend([17, 31, 73, 47, 23].iter());

        Puzzle {
            list,
            lengths,
            cursor: 0,
            skip: 0,
        }
    }

    fn hash(&mut self) {
        // run one round of hash
        for &length in self.lengths.iter() {
            let (head, tail) = if self.cursor + (length as usize) < self.list.len() {
                (
                    &mut self.list[self.cursor..self.cursor + length as usize],
                    None,
                )
            } else {
                let (front, back) = self.list.split_at_mut(self.cursor);
                let back_len = back.len();
                (back, Some(&mut front[0..(length as usize - back_len)]))
            };

            reverse(head, tail);

            self.cursor += length as usize + self.skip;
            self.cursor %= self.list.len();

            self.skip += 1;
        }
    }

    fn solve_part1(mut self) -> u16 {
        self.hash();
        self.list[0] as u16 * self.list[1] as u16
    }

    fn solve_part2(mut self) -> String {
        for _ in 0..64 {
            self.hash();
        }

        self.list
            .chunks(16)
            .map(|win| {
                win.iter()
                    .copied()
                    .reduce(std::ops::BitXor::bitxor)
                    .unwrap()
            })
            .map(|val| format!("{val:02x}"))
            .join("")
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> u16 {
    Puzzle::new(255, input).solve_part1()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> String {
    Puzzle::new2(255, input).solve_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(Puzzle::new(4, "3,4,1,5").solve_part1(), 12);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            Puzzle::new2(255, "").solve_part2(),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            Puzzle::new2(255, "AoC 2017").solve_part2(),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            Puzzle::new2(255, "1,2,3").solve_part2(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            Puzzle::new2(255, "1,2,4").solve_part2(),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
