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

#[derive(Debug, Clone)]
struct TreeNode {
    _name: String,
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
            _name: from.0,
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

    /// returns an option of the unbalanced node the corrected self weight
    fn unbalanced_child_and_balancing_weight(&self) -> Option<(TreeNode, usize)> {
        // if there are no children
        if self.children.is_empty() {
            return None;
        }

        if self
            .children
            .iter()
            .skip(1)
            .all(|c| c.total_weight() == self.children[0].total_weight())
        {
            return None;
        }

        // hashmap to group children by their total weights
        let weights: HashMap<usize, Vec<&TreeNode>> =
            self.children.iter().fold(HashMap::new(), |mut map, c| {
                map.entry(c.total_weight()).or_default().push(c);
                map
            });

        // if there are more than two children
        // we can easily tell which is unbalanced
        if self.children.len() > 2 {
            // based on rules of the aoc problem
            // the expect below is safe
            // as there always be one imbalanced node
            // edit later with test data
            // it is not safe haha
            // because we could be inside of a balanced tree
            // doing an early return above for that option and it worked :)))))))))

            let (min_weight, unbalanced_child) = weights
                .iter()
                .find(|c| c.1.len() == 1)
                .expect(format!("unbalanced_child not found in {:#?}", weights).as_str());

            let (maj_weight, _) = weights
                .iter()
                .find(|c| c.1.len() > 1)
                .expect("maj_weight not found");

            let target_weight =
                unbalanced_child[0].weight as isize + (*maj_weight as isize - *min_weight as isize);

            return Some((unbalanced_child[0].clone(), target_weight as usize));
        }

        // if there are only two children
        // or one child
        // we are going to have to recur into those children/that child
        // to find imbalance
        if self.children.len() < 3 {
            if let Some(ubc) = self
                .children
                .iter()
                .find(|c| c.unbalanced_child_and_balancing_weight().is_some())
            {
                return ubc.unbalanced_child_and_balancing_weight();
            } else {
                return None;
            }
        }

        None
    }

    fn deepest_unbalanced_child(&self) -> Option<(TreeNode, usize)> {
        if let Some((curr_ubc, _)) = self.unbalanced_child_and_balancing_weight() {
            if curr_ubc.unbalanced_child_and_balancing_weight().is_none() {
                return self.unbalanced_child_and_balancing_weight();
            } else {
                return curr_ubc.deepest_unbalanced_child();
            }
        }
        None
    }
}

#[allow(dead_code)]
pub fn part_one() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();
    println!(
        "part_one={:#?}\n...\nruntime={:?}",
        {
            let hashnodes = include_str!("../inputs/day07.txt")
                .lines()
                .filter(|l| !l.starts_with("//"))
                .map(|s| Node::from_str(s))
                .collect::<Result<HashNodes, _>>()?;

            hashnodes.root().expect("root not found").0
        },
        start.elapsed()
    );
    Ok(())
}

#[allow(dead_code)]
pub fn part_two() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();
    println!(
        "\npart_two={:#?}\n...\nruntime={:?}",
        {
            let hashnodes = include_str!("../inputs/day07.txt")
                .lines()
                .filter(|l| !l.starts_with("//"))
                .map(|s| Node::from_str(s))
                .collect::<Result<HashNodes, _>>()?;

            let treenode = TreeNode::try_from(hashnodes).expect("failed to build treenode");
            treenode.deepest_unbalanced_child().expect("shoot").1
        },
        start.elapsed()
    );
    Ok(())
}
