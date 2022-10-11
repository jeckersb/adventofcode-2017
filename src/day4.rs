use std::collections::BTreeSet;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    let mut passphrases = Vec::new();

    for line in input.lines() {
        let mut words = Vec::new();

        for n in line.split_whitespace() {
            words.push(n.to_string());
        }

        passphrases.push(words)
    }

    passphrases
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> u32 {
    let mut valid = 0;

    'outer: for passphrase in input.iter() {
        let mut set = BTreeSet::new();

        for word in passphrase.iter() {
            if !set.insert(word) {
                continue 'outer;
            }
        }

        valid += 1;
    }

    valid
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<String>]) -> u32 {
    let mut valid = 0;

    'outer: for passphrase in input.iter() {
        let mut set = BTreeSet::new();

        for word in passphrase.iter() {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let word = String::from_iter(chars);

            if !set.insert(word) {
                continue 'outer;
            }
        }

        valid += 1;
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator("aa bb cc dd ee")), 1);
        assert_eq!(solve_part1(&input_generator("aa bb cc dd aa")), 0);
        assert_eq!(solve_part1(&input_generator("aa bb cc dd aaa")), 1);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator("abcde fghij")), 1);
        assert_eq!(solve_part2(&input_generator("abcde xyz ecdab")), 0);
        assert_eq!(solve_part2(&input_generator("a ab abc abd abf abj")), 1);
        assert_eq!(solve_part2(&input_generator("iiii oiii ooii oooi oooo")), 1);
        assert_eq!(solve_part2(&input_generator("oiii ioii iioi iiio")), 0);
    }
}
