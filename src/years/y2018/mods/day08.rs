use std::collections::VecDeque;

fn nums() -> Vec<usize> {
    include_str!("../inputs/day08.txt")
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("failed to parse to usize"))
        .collect()
}

enum Instr {
    Node,
    MetaData(usize),
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let nums = nums();
    let mut p: usize = 0;
    let mut total: usize = 0;
    let mut stack: VecDeque<Instr> = VecDeque::from([Instr::Node]);

    while let Some(instr) = stack.pop_front() {
        match instr {
            Instr::Node => {
                stack.push_front(Instr::MetaData(nums[p + 1]));
                for _ in 0..nums[p] {
                    stack.push_front(Instr::Node);
                }
                p += 2;
            }
            Instr::MetaData(length) => {
                for _ in 0..length {
                    total += nums[p];
                    p += 1;
                }
            }
        }
    }

    println!("part_one={:#?}", total);
    println!("runtime={:?}", start.elapsed());
}

#[derive(Debug)]
struct Node {
    id: usize,
    children: Vec<usize>,
    metadeta: Vec<usize>,
}

impl Node {
    fn value(&self, nodes: &Vec<Node>) -> usize {
        if self.children.is_empty() {
            return self.metadeta.iter().sum();
        }

        self.metadeta
            .iter()
            .map(|child_index| {
                if let Some(child_id) = self.children.get(*child_index - 1) {
                    nodes
                        .iter()
                        .find(|child| child.id == *child_id)
                        .expect("child not found")
                        .value(nodes)
                } else {
                    0
                }
            })
            .sum()
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();

    let nums = nums();
    let mut p: usize = 0;
    let mut nodes: Vec<Node> = Vec::new();
    let mut node_id: usize = 0;
    // contains id of node and instruction
    // initializing with None as the root has no parent
    let mut stack: VecDeque<(Option<usize>, Instr)> = VecDeque::from([(None, Instr::Node)]);

    while let Some((parent_node_id_option, instr)) = stack.pop_front() {
        match instr {
            Instr::Node => {
                // add new node to list
                nodes.push(Node {
                    id: node_id,
                    children: Vec::new(),
                    metadeta: Vec::new(),
                });

                // add new node to child list of existing
                if let Some(parent_node_id) = parent_node_id_option {
                    if let Some(parent_node) = nodes.iter_mut().find(|n| n.id == parent_node_id) {
                        parent_node.children.push(node_id);
                    }
                }

                // regular instruction adding
                stack.push_front((Some(node_id), Instr::MetaData(nums[p + 1])));
                for _ in 0..nums[p] {
                    stack.push_front((Some(node_id), Instr::Node));
                }
                p += 2;
                node_id += 1;
            }
            Instr::MetaData(length) => {
                for _ in 0..length {
                    // add new metadata to list of existing
                    if let Some(parent_node_id) = parent_node_id_option {
                        if let Some(parent_node) = nodes.iter_mut().find(|n| n.id == parent_node_id)
                        {
                            parent_node.metadeta.push(nums[p]);
                        }
                    }

                    p += 1;
                }
            }
        }
    }

    let result = nodes
        .iter()
        .find(|n| n.id == 0)
        .expect("root not found")
        .value(&nodes);

    println!("part_two={:#?}", result);
    println!("runtime={:?}", start.elapsed());
}
