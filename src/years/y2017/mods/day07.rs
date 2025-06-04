use std::{collections::HashMap, error::Error, fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum NodeError {
    OpenParenthesisNotFound,
    CloseParenthesisNotFound,
    NameNotFound,
    ParseWeightError(ParseIntError),
}

impl Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::OpenParenthesisNotFound => {
                write!(f, "open parenthesis needed to identify weight")
            }
            NodeError::CloseParenthesisNotFound => {
                write!(f, "close parenthesis needed to identify weight")
            }
            NodeError::NameNotFound => {
                write!(f, "name not found before open parenthesis")
            }
            NodeError::ParseWeightError(_) => write!(
                f,
                "value found between parenthesis failed to parse to usize"
            ),
        }
    }
}

impl Error for NodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NodeError::ParseWeightError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct NodeData {
    weight: usize,
    children: Vec<String>,
}

struct Node(String, NodeData);
impl std::str::FromStr for Node {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opp = s
            .find("(")
            .ok_or_else(|| NodeError::OpenParenthesisNotFound)?;

        let name = s[..opp].trim().to_string();

        if name.is_empty() {
            return Result::Err(Box::new(NodeError::NameNotFound));
        }

        let cpp = s
            .find(")")
            .ok_or_else(|| NodeError::CloseParenthesisNotFound)?;

        let weight: usize = s[opp + 1..cpp]
            .parse()
            .map_err(|e| NodeError::ParseWeightError(e))?;

        let mut children: Vec<String> = Vec::new();

        if let Some((_, children_str)) = s.split_once("->") {
            children.extend(
                children_str
                    .split(",")
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<String>>(),
            )
        }

        Ok(Node(name, NodeData { weight, children }))
    }
}

type HashNodes = HashMap<String, NodeData>;
impl FromIterator<Node> for HashNodes {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        iter.into_iter()
            .map(|Node(name, node_data)| (name, node_data))
            .collect()
    }
}

trait Root {
    fn root(&self) -> Option<TupleNode>;
}

type TupleNode = (String, NodeData);
impl Root for HashNodes {
    fn root(&self) -> Option<TupleNode> {
        let children: Vec<&String> = self
            .values()
            .filter(|x| !x.children.is_empty())
            .flat_map(|x| &x.children)
            .collect();
        self.iter()
            .find(|k| !children.contains(&k.0))
            .map(|(name, data)| (name.clone(), data.clone()))
    }
}

#[derive(Debug, Default)]
struct TreeNode {
    name: String,
    weight: usize,
    children: Vec<Box<TreeNode>>,
}

#[derive(Debug)]
enum TreeNodeError {
    RootNotFound,
}

impl Display for TreeNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "root not found")
    }
}

impl Error for TreeNodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl TryFrom<HashNodes> for TreeNode {
    type Error = TreeNodeError;
    fn try_from(value: HashNodes) -> Result<Self, Self::Error> {
        let tuple_node = value.root().ok_or_else(|| TreeNodeError::RootNotFound)?;
        Ok(TreeNode::new(tuple_node, &value))
    }
}

impl TreeNode {
    fn new(from: TupleNode, nodes: &HashNodes) -> Self {
        TreeNode {
            name: from.0,
            weight: from.1.weight,
            children: from
                .1
                .children
                .into_iter()
                .map(|c| {
                    Box::new(TreeNode::new(
                        nodes
                            .iter()
                            .find(|(name, _)| **name == c)
                            .map(|(name, data)| (name.clone(), data.clone()))
                            .unwrap(),
                        nodes,
                    ))
                })
                .collect(),
        }
    }

    fn total_weight(&self) -> usize {
        self.weight
            + self
                .children
                .iter()
                .map(|c| c.total_weight())
                .sum::<usize>()
    }
}

#[allow(dead_code)]
pub fn part_one() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();
    println!(
        "{:#?}\n...\nruntime={:?}",
        {
            let hashnodes = include_str!("../inputs/day07.txt")
                .lines()
                .map(|s| Node::from_str(s))
                .collect::<Result<HashNodes, _>>()?;

            let treenode = TreeNode::try_from(hashnodes)?;

            let treenode_weights = treenode
                .children
                .iter()
                .map(|c| c.total_weight())
                .collect::<Box<[usize]>>();

            let mut out_msg = String::new();
            if treenode_weights[0]
                != treenode_weights.iter().sum::<usize>() / treenode_weights.len()
            {
                out_msg = "not balanced".to_string();
            }
            out_msg
        },
        start.elapsed()
    );
    Ok(())
}

#[allow(dead_code)]
pub fn part_two() {}
