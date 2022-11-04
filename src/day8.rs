use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Sub};

use anyhow::{anyhow, Result};
use nom::sequence::preceded;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i32, newline, space1},
    combinator::{map, opt},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

type RegOp = fn(i32, i32) -> i32;

struct Instruction<'a> {
    reg: &'a str,
    op: RegOp,
    operand: i32,
    cond: Condition<'a>,
}

type OrdOp = fn(Ordering) -> bool;

#[derive(Debug, PartialEq)]
struct Condition<'a> {
    reg: &'a str,
    op: OrdOp,
    operand: i32,
}

struct Cpu<'a> {
    inst: Vec<Instruction<'a>>,
    reg: HashMap<&'a str, i32>,
    max: i32,
}

impl<'a> Instruction<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(alpha1, space1),
                terminated(Self::parse_op, space1),
                terminated(i32, space1),
                Condition::parse,
            )),
            |(reg, op, operand, cond)| Self {
                reg,
                op,
                operand,
                cond,
            },
        )(input)
    }

    fn parse_op(input: &'a str) -> IResult<&str, RegOp> {
        alt((
            map(tag("inc"), |_| i32::add as RegOp),
            map(tag("dec"), |_| i32::sub as RegOp),
        ))(input)
    }
}

impl<'a> Condition<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = preceded(tag("if"), space1)(input)?;
        map(
            tuple((
                terminated(alpha1, space1),
                terminated(Self::parse_ordering, space1),
                i32,
            )),
            |(reg, op, operand)| Self { reg, op, operand },
        )(input)
    }

    fn parse_ordering(input: &'a str) -> IResult<&str, OrdOp> {
        alt((
            map(tag(">="), |_| Ordering::is_ge as OrdOp),
            map(tag("<="), |_| Ordering::is_le as OrdOp),
            map(tag(">"), |_| Ordering::is_gt as OrdOp),
            map(tag("<"), |_| Ordering::is_lt as OrdOp),
            map(tag("=="), |_| Ordering::is_eq as OrdOp),
            map(tag("!="), |_| Ordering::is_ne as OrdOp),
        ))(input)
    }
}

impl<'a> Cpu<'a> {
    fn new(input: &'a str) -> Result<Self> {
        match Self::parse_instructions(input) {
            Ok((_, inst)) => {
                let reg = HashMap::new();
                Ok(Self {
                    inst,
                    reg,
                    max: i32::MIN,
                })
            }
            Err(e) => Err(anyhow!("Unable to parse instructions: {}", e)),
        }
    }

    fn parse_instructions(input: &'a str) -> IResult<&str, Vec<Instruction>> {
        many1(terminated(Instruction::parse, opt(newline)))(input)
    }

    fn exec(&mut self) {
        for inst in self.inst.iter() {
            let cond_reg_val = self.reg.entry(inst.cond.reg).or_default().to_owned();
            if (inst.cond.op)(cond_reg_val.cmp(&inst.cond.operand)) {
                let reg_entry = self.reg.entry(inst.reg).or_default();
                *reg_entry = (inst.op)(*reg_entry, inst.operand);
                self.max = self.max.max(*reg_entry);
            }
        }
    }

    fn largest_register(&self) -> Option<i32> {
        self.reg.values().copied().max()
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut cpu = Cpu::new(input).unwrap();
    cpu.exec();
    cpu.largest_register().unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut cpu = Cpu::new(input).unwrap();
    cpu.exec();
    cpu.max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_condition_greater_than() {
        assert_eq!(
            Condition::parse("if a > 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_gt as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn test_parse_condition_less_than() {
        assert_eq!(
            Condition::parse("if a < 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_lt as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn test_parse_condition_greater_than_or_equal() {
        assert_eq!(
            Condition::parse("if a >= 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_ge as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn test_parse_condition_less_than_or_equal() {
        assert_eq!(
            Condition::parse("if a <= 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_le as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn test_parse_condition_equal() {
        assert_eq!(
            Condition::parse("if a == 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_eq as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn test_parse_condition_not_equal() {
        assert_eq!(
            Condition::parse("if a != 1"),
            Ok((
                "",
                Condition {
                    reg: "a",
                    op: Ordering::is_ne as OrdOp,
                    operand: 1,
                }
            ))
        );
    }

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(
                "b inc 5 if a > 1\n\
		 a inc 1 if b < 5\n\
		 c dec -10 if a >= 1\n\
		 c inc -20 if c == 10"
            ),
            1
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "b inc 5 if a > 1\n\
		 a inc 1 if b < 5\n\
		 c dec -10 if a >= 1\n\
		 c inc -20 if c == 10"
            ),
            10
        );
    }
}
