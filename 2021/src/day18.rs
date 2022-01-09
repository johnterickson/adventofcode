use std::{str::Chars, iter::Peekable, fmt::Display, collections::BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NodeId(usize);


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    Literal(u64),
    Pair(NodeId, NodeId)
}

impl Node {
    fn parse(parent: Option<NodeId>, chars: &mut Peekable<Chars>, tree: &mut NodeTree) -> NodeId {
        let id = tree.cur;
        tree.cur = NodeId(tree.cur.0 + 1);

        let node = if chars.peek().unwrap().is_digit(10) {
            let mut n = 0u64;
            loop {
                let digit = if let Some(c) = chars.peek() {
                    c.to_digit(10)
                } else {
                    None
                };

                if let Some(digit) = digit {
                    let digit: u64 = digit.into();
                    let _ = chars.next().unwrap();
                    n *= 10;
                    n = n + digit;
                    continue;
                }

                break Node::Literal(n);
            }
        } else {
            assert_eq!(Some('['), chars.next());
            let left_id = Node::parse(Some(id), chars, tree);
    
            assert_eq!(Some(','), chars.next());
            let right_id = Node::parse(Some(id), chars, tree);
    
            assert_eq!(Some(']'), chars.next());
            Node::Pair(left_id, right_id)
        };
        tree.nodes.insert(id, node);
        if let Some(parent) = parent {
            tree.parents.insert(id, parent);
        }
        id
    }
}

struct NodeTree {
    cur: NodeId,
    nodes: BTreeMap<NodeId, Node>,
    parents: BTreeMap<NodeId, NodeId>,
}

impl NodeTree {
    fn find_explode(&self, depth: usize, id: &NodeId) -> Option<NodeId> {
        match &self.nodes[&id] {
            Node::Literal(_) => None,
            Node::Pair(l, r) => {
                if depth >= 4 {
                    Some(*id)
                }
                else if let Some(id) = self.find_explode(depth + 1, l) {
                    Some(id)
                }
                else if let Some(id) = self.find_explode(depth + 1, r) {
                    Some(id)
                } else {
                    None
                }
            },
        }
    } 

    fn in_order<F: FnMut(NodeId)>(&self, id: NodeId, f: &mut F) {
        let node = self.nodes[&id];
        println!("{:?} -> {:?}", id, &node);

        match node {
            Node::Literal(_) => {},
            Node::Pair(l, _) => self.in_order(l, f),
        }

        f(id);

        match node {
            Node::Literal(_) => {},
            Node::Pair(_, r) => self.in_order(r, f),
        }
    }

    fn explode(&mut self, id: NodeId) {
        // replace with a zero
        let node = self.nodes.insert(id, Node::Literal(0)).unwrap();
        println!("{}", &self);

        if let Node::Pair(l_id, r_id) = node {
            let mut remove = |parent_id, id| {
                let n = self.nodes.remove(&id).unwrap();
                assert_eq!(Some(parent_id), self.parents.remove(&id));
                match n {
                    Node::Literal(n) => n,
                    Node::Pair(_, _) => panic!(),
                }
            };
            
            let l = remove(id, l_id);
            let r = remove(id, r_id);

            println!("{}", &self);

            let nodes_in_order = {
                let mut nodes_in_order = Vec::new();
                self.in_order(NodeId(0), &mut |n| {
                    match self.nodes[&n] {
                        Node::Literal(v) => nodes_in_order.push((n,v)),
                        Node::Pair(_, _) => {},
                    }
                });
                nodes_in_order
            };

            println!("{:?}", &nodes_in_order);
            let index = nodes_in_order.iter().position(|(i,v)| i == &id).unwrap();
            assert_eq!(&Node::Literal(0), &self.nodes[&nodes_in_order[index].0]);

            if let Some(previous_index) = index.checked_sub(1) {
                if let Some((prev_id, prev_val)) = nodes_in_order.get(previous_index) {
                    match self.nodes.get_mut(&prev_id).unwrap() {
                        Node::Literal(ref mut n) => *n += l,
                        Node::Pair(_, _) => panic!(),
                    }
                }
            }
            if let Some(next_index) = index.checked_add(1) {
                if let Some((next_id, next_val)) = nodes_in_order.get(next_index) {
                    match self.nodes.get_mut(&next_id).unwrap() {
                        Node::Literal(ref mut n) => *n += r,
                        Node::Pair(_, _) => panic!(),
                    }
                }
            }
        } else {
            panic!();
        }
    }

    fn fmt_node(&self, id: &NodeId, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.nodes[id] {
            Node::Literal(n) => write!(f,"{}", n),
            Node::Pair(left, right) => {
                write!(f,"[")?;
                self.fmt_node(&left, f)?;
                write!(f,",")?;
                self.fmt_node(&right, f)?;
                write!(f,"]")
            }
        }
    }
}

impl Display for NodeTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_node(&NodeId(0), f)
    }
}


#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<NodeTree> {
    input.trim().lines().map(|line| {
        let mut tree = NodeTree {
            cur: NodeId(0),
            nodes: BTreeMap::new(),
            parents: BTreeMap::new(),
        };
        let mut chars = line.trim().chars().peekable();
        assert_eq!(NodeId(0), Node::parse(None, &mut chars, &mut tree));
        tree
    }).collect()
}

#[aoc(day18, part1)]
fn part1(pairs: &Vec<NodeTree>) -> usize { 
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn parsing() {
       let nums =
           "[1,2]
           [[1,2],3]
           [9,[8,7]]
           [[1,9],[8,5]]
           [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
           [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
           [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let parsed = parse_input(nums);
        for (parsed, expected) in parsed.iter().zip(nums.lines()) {
            let mut formatted = String::new();
            write!(&mut formatted, "{}", parsed).unwrap();
            assert_eq!(formatted.as_str(), expected.trim());
        }
    }

    fn test_explode(input: &str, expected: &str) {
        dbg!(input, expected);
        let mut input = parse_input(input);
        let input = &mut input[0];
        if let Some(to_explode) = input.find_explode(0, &NodeId(0)) {
            if let Node::Pair(l, r) = input.nodes[&to_explode] {
                // assert_eq!(Node::Literal(9), input.nodes[&l]);
                // assert_eq!(Node::Literal(8), input.nodes[&r]);
            } else {
                panic!("Expected pair but found: {:?} -> {:?}", to_explode, input.nodes[&to_explode]);
            }

            input.explode(to_explode);

            let mut formatted = String::new();
            write!(&mut formatted, "{}", input).unwrap();
            assert_eq!(formatted.as_str(), expected.trim());
        } else {
            panic!();
        }
    }

    #[test]
    fn part1_example1() {
        test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        test_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        
    }
}