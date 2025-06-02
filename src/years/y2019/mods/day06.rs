use std::collections::VecDeque;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    from: String,
    to: String,
}

fn nodes() -> Box<[Node]> {
    include_str!("../inputs/day06.txt")
        .lines()
        .map(|l| {
            let (from_str, to_str) = l.split_once(")").expect("failed to split once");
            let (from, to) = (from_str.to_string(), to_str.to_string());
            Node { from, to }
        })
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let start = Instant::now();
    // actual
    {
        let nodes = nodes();
        // println!("nodes length={}", nodes.len());
        // node and depth
        let root = nodes
            .iter()
            .find(|node| node.from == "COM")
            .expect("root not found");
        let mut queue = VecDeque::from([(root, 0)]);
        let mut total = 0;

        while let Some((node, depth)) = queue.pop_front() {
            // println!("# node={:?}, depth={}", node, depth);
            total += depth + 1;
            // println!(" - total={}", total);
            // println!("## tos");
            queue.extend(nodes.iter().filter(|to| node.to == to.from).map(|to| {
                let result = (to, depth + 1);
                // println!(" - {:?}", result);
                result
            }));
        }
        print!("part_one = {}", total);
    }
    println!(" ... runtime = {:?}", start.elapsed());
}

/// takes from_node or none if desired node is root
/// starts at to_node and finds its way back
/// returns none if either node is not contained in list
fn path_from(
    from_node_option: Option<&Node>,
    to_node: &Node,
    nodes: &Box<[Node]>,
) -> Option<Vec<Node>> {
    // if either node isn't in our list
    // then no path exists
    if let Some(from_node) = from_node_option {
        if !nodes.contains(from_node) || !nodes.contains(to_node) {
            return None;
        }
    }

    let mut result = vec![];
    let mut queue = VecDeque::from([to_node]);

    // regular logic
    while let Some(this_node) = queue.pop_front() {
        result.push(this_node.clone());

        // if reached desired node result is complete
        if let Some(from_node) = from_node_option {
            if this_node == from_node {
                return Some(result);
            }
        }

        // regular logic
        if let Some(next_node) = nodes
            .iter()
            .find(|next_node| next_node.to == this_node.from)
        {
            queue.push_back(next_node);
        }
    }
    // i don't think we can reach this
    Some(result)
}

fn common_ancestor(node1: &Node, node2: &Node, nodes: &Box<[Node]>) -> Node {
    let path1 = path_from(None, node1, nodes).expect("failed to find path");
    let path2 = path_from(None, node2, nodes).expect("failed to find path");
    let common = path1
        .iter()
        .find(|node| path2.contains(&node))
        .expect("failed to find common");
    common.clone()
}

#[allow(dead_code)]
pub fn part_two() {
    let start = Instant::now();
    // actual
    {
        let nodes = nodes();
        let santa = nodes
            .iter()
            .find(|node| node.to == "SAN")
            .expect("santa not found")
            .to_owned();
        let you = nodes
            .iter()
            .find(|node| node.to == "YOU")
            .expect("you not found")
            .to_owned();

        let common_ancestor = common_ancestor(&santa, &you, &nodes);

        let santa_path_to_common =
            path_from(Some(&common_ancestor), &santa, &nodes).expect("path not found");
        let you_path_to_common =
            path_from(Some(&common_ancestor), &you, &nodes).expect("path not found");

        let santa_distance_to_common = santa_path_to_common.len() - 2;
        let you_distance_to_common = you_path_to_common.len() - 2;

        // println!("{:?}", santa_distance_to_common);
        // println!("{:?}", you_distance_to_common);

        let distance_from_each_other = santa_distance_to_common + you_distance_to_common;

        print!("part_two = {}", distance_from_each_other);
    }
    println!(" ... runtime = {:?}", start.elapsed());
}
