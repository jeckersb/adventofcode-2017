use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{anychar, char},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Thing {
    Group(Vec<Thing>),
    Garbage(i32),
}

impl Thing {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_garbage, Self::parse_group))(input)
    }

    fn parse_garbage(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("<"),
                many0(alt((
                    map(pair(char('!'), anychar), |_| 0),
                    map(is_not("!>"), |s: &str| s.len() as i32),
                ))),
                tag(">"),
            ),
            |counts| Self::Garbage(counts.iter().sum()),
        )(input)
    }

    fn parse_group(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("{"),
                many0(terminated(Thing::parse, opt(char(',')))),
                tag("}"),
            ),
            Self::Group,
        )(input)
    }

    fn score(&self) -> i32 {
        self._score(1)
    }

    fn _score(&self, depth: i32) -> i32 {
        match self {
            Thing::Garbage(_) => 0,
            Thing::Group(things) => {
                depth
                    + things
                        .iter()
                        .map(|thing| thing._score(depth + 1))
                        .sum::<i32>()
            }
        }
    }

    fn garbage_count(&self) -> i32 {
        match self {
            Thing::Garbage(count) => *count,
            Thing::Group(things) => things
                .iter()
                .map(|thing| thing.garbage_count())
                .sum::<i32>(),
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let (_, thing) = Thing::parse(input).unwrap();
    thing.score()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let (_, thing) = Thing::parse(input).unwrap();
    thing.garbage_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1_garbage() {
        assert_eq!(Thing::parse_garbage("<>"), Ok(("", Thing::Garbage(0))));
        assert_eq!(
            Thing::parse_garbage("<random characters>"),
            Ok(("", Thing::Garbage(17)))
        );
        assert_eq!(Thing::parse_garbage("<<<<>"), Ok(("", Thing::Garbage(3))));
        assert_eq!(Thing::parse_garbage("<{!>}>"), Ok(("", Thing::Garbage(2))));
        assert_eq!(Thing::parse_garbage("<!!>"), Ok(("", Thing::Garbage(0))));
        assert_eq!(Thing::parse_garbage("<!!!>>"), Ok(("", Thing::Garbage(0))));
        assert_eq!(
            Thing::parse_garbage("<{o\"i!a,<{i<a>"),
            Ok(("", Thing::Garbage(10)))
        );
    }

    #[test]
    fn examples_part1_groups() {
        // {}, 1 group.
        assert_eq!(Thing::parse("{}"), Ok(("", Thing::Group(vec![]))));

        // {{{}}}, 3 groups.
        assert_eq!(
            Thing::parse("{{{}}}"),
            Ok((
                "",
                Thing::Group(vec![Thing::Group(vec![Thing::Group(vec![])])])
            ))
        );

        // {{},{}}, also 3 groups.
        assert_eq!(
            Thing::parse("{{},{}}"),
            Ok((
                "",
                Thing::Group(vec![Thing::Group(vec![]), Thing::Group(vec![])])
            ))
        );

        // {{{},{},{{}}}}, 6 groups.
        assert_eq!(
            Thing::parse("{{{},{},{{}}}}"),
            Ok((
                "",
                Thing::Group(vec![Thing::Group(vec![
                    Thing::Group(vec![]),
                    Thing::Group(vec![]),
                    Thing::Group(vec![Thing::Group(vec![])])
                ])])
            ))
        );

        // {<{},{},{{}}>}, 1 group (which itself contains garbage).
        assert_eq!(
            Thing::parse("{<{},{},{{}}>}"),
            Ok(("", Thing::Group(vec![Thing::Garbage(10)])))
        );

        // {<a>,<a>,<a>,<a>}, 1 group.
        assert_eq!(
            Thing::parse("{<a>,<a>,<a>,<a>}"),
            Ok((
                "",
                Thing::Group(vec![
                    Thing::Garbage(1),
                    Thing::Garbage(1),
                    Thing::Garbage(1),
                    Thing::Garbage(1)
                ])
            ))
        );

        // {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
        assert_eq!(
            Thing::parse("{{<a>},{<a>},{<a>},{<a>}}"),
            Ok((
                "",
                Thing::Group(vec![
                    Thing::Group(vec![Thing::Garbage(1)]),
                    Thing::Group(vec![Thing::Garbage(1)]),
                    Thing::Group(vec![Thing::Garbage(1)]),
                    Thing::Group(vec![Thing::Garbage(1)]),
                ])
            ))
        );

        // {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).
        assert_eq!(
            Thing::parse("{{<!>},{<!>},{<!>},{<a>}}"),
            Ok((
                "",
                Thing::Group(vec![Thing::Group(vec![Thing::Garbage(13)])])
            ))
        );
    }

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1("{}"), 1);
        assert_eq!(solve_part1("{{{}}}"), 6);
        assert_eq!(solve_part1("{{},{}}"), 5);
        assert_eq!(solve_part1("{{{},{},{{}}}}"), 16);
        assert_eq!(solve_part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(solve_part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(solve_part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(solve_part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2("<>"), 0);
        assert_eq!(solve_part2("<random characters>"), 17);
        assert_eq!(solve_part2("<<<<>"), 3);
        assert_eq!(solve_part2("<{!>}>"), 2);
        assert_eq!(solve_part2("<!!>"), 0);
        assert_eq!(solve_part2("<!!!>>"), 0);
        assert_eq!(solve_part2("<{o\"i!a,<{i<a>"), 10);
    }
}
