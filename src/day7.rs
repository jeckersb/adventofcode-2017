/*
use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i32, newline, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Node<'a> {
    name: &'a str,
    weight: i32,
    total_weight: i32,
    parent: Option<&'a str>,
    children: Option<Vec<&'a str>>,
}

impl<'a> Node<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(Self::parse_name, space1),
                terminated(Self::parse_weight, opt(space1)),
                opt(Self::parse_children),
            )),
            |(name, weight, children)| Self {
                total_weight: weight,
                parent: None,
                name,
                weight,
                children,
            },
        )(input)
    }

    fn parse_name(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn parse_weight(input: &str) -> IResult<&str, i32> {
        delimited(tag("("), i32, tag(")"))(input)
    }

    fn parse_children(input: &'a str) -> IResult<&str, Vec<&'a str>> {
        let (input, _) = preceded(tag("->"), space1)(input)?;
        separated_list1(tuple((tag(","), space1)), Self::parse_name)(input)
    }
}

struct Graph<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Result<Self> {
        match Self::parse_nodes(input) {
            Ok((_, nodes)) => {
                let mut hashmap = HashMap::new();
                for node in nodes {
                    hashmap.insert(node.name, node);
                }
                Ok(Graph(hashmap))
            }
            Err(e) => Err(anyhow!("Unable to parse graph: {}", e)),
        }
    }

    fn parse_nodes(input: &'a str) -> IResult<&str, Vec<Node>> {
        many1(terminated(Node::parse, opt(newline)))(input)
    }

    fn root(&self) -> &str {
        let all_nodes: HashSet<&str> = self.0.keys().copied().collect();
        let mut childset = HashSet::new();

        for (_, v) in self.0.iter() {
            if let Some(ref children) = v.children {
                for &child in children {
                    childset.insert(child);
                }
            } else {
                // leaf node
                childset.insert(v.name);
            }
        }

        let roots: Vec<_> = all_nodes.difference(&childset).copied().collect();
        assert!(roots.len() == 1);

        roots[0]
    }

    fn adjusted_weight(&self) -> i32 {
        let mut weights = HashMap::new();
        let mut stack = Vec::new();
        let root = self.0.get(self.root()).unwrap();

        // pass 1: compute weights
        stack.push((root, false, None));
        while let Some((cur, seen, parent)) = stack.pop() {
            if !seen {
                stack.push((cur, true, parent));
                cur.parent = parent;

                if let Some(children) = &cur.children {
                    for &child in children {
                        stack.push((self.0.get_mut(child).unwrap(), false, Some(cur.name)));
                    }
                }
            } else {
                let mut weight = cur.weight;
                if let Some(children) = &cur.children {
                    for &child in children {
                        cur.total_weight += self.0.get(child).unwrap().total_weight;
                    }
                }
                weights.insert(cur.name, weight);
            }
        }

        println!("{:?}", weights);

        // pass 2: find the wrong weight
        let mut target_weight = 0;
        let mut cur: &Node = root;

        loop {
            let mut count_by_weight: HashMap<i32, Vec<&str>> = HashMap::new();

            for child in cur.children.as_ref().unwrap() {
                let child_node = self.0.get(child).unwrap();

                count_by_weight
                    .entry(*weights.get(child_node.name).unwrap())
                    .and_modify(|children| children.push(child_node.name))
                    .or_insert_with(|| vec![child_node.name]);
            }

            // if all children are the same weight then we've found
            // the node we need to adjust
            if count_by_weight.keys().len() == 1 {
                println!(
                    "cur.weight={} target_weight={} weights(cur)={}",
                    cur.weight,
                    target_weight,
                    weights.get(cur.name).unwrap()
                );
                return cur.weight + (target_weight - *weights.get(cur.name).unwrap());
            }

            // otherwise walk up the tree to the one that is different from the rest
            for (k, v) in count_by_weight {
                if v.len() == 1 {
                    //wrong
                    cur = self.0.get(v[0]).unwrap();
                } else {
                    // right, update target
                    target_weight = k;
                }
            }
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> String {
    let graph = Graph::new(input).unwrap();
    graph.root().to_string()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> i32 {
    Graph::new(input).unwrap().adjusted_weight()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_parse_name() {
        assert_eq!(Node::parse_name("abcd"), Ok(("", "abcd")));
    }

    #[test]
    fn test_node_parse_weight() {
        assert_eq!(Node::parse_weight("(42)"), Ok(("", 42i32)));
    }

    #[test]
    fn test_node_parse_children_populated() {
        assert_eq!(
            Node::parse_children("-> abcd, efgh"),
            Ok(("", vec!["abcd", "efgh"]))
        );
    }

    #[test]
    fn test_node_parse_children_empty() {
        assert_eq!(
            Node::parse("abcd (42)"),
            Ok((
                "",
                Node {
                    name: "abcd",
                    weight: 42,
                    total_weight: 42,
                    parent: None,
                    children: None,
                }
            ))
        );
    }

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(
                "pbga (66)\n\
         xhth (57)\n\
         ebii (61)\n\
         havc (66)\n\
         ktlj (57)\n\
         fwft (72) -> ktlj, cntj, xhth\n\
         qoyq (66)\n\
         padx (45) -> pbga, havc, qoyq\n\
         tknk (41) -> ugml, padx, fwft\n\
         jptl (61)\n\
         ugml (68) -> gyxo, ebii, jptl\n\
         gyxo (61)\n\
         cntj (57)"
            ),
            "tknk"
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "pbga (66)\n\
         xhth (57)\n\
         ebii (61)\n\
         havc (66)\n\
         ktlj (57)\n\
         fwft (72) -> ktlj, cntj, xhth\n\
         qoyq (66)\n\
         padx (45) -> pbga, havc, qoyq\n\
         tknk (41) -> ugml, padx, fwft\n\
         jptl (61)\n\
         ugml (68) -> gyxo, ebii, jptl\n\
         gyxo (61)\n\
         cntj (57)"
            ),
            60
        );
    }
}
*/
