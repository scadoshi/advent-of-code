use std::{error::Error, num::ParseIntError, str::FromStr};

#[derive(Debug, Default)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

#[derive(Debug)]
enum ProgramCreationError {
    OpenBracketNotFound,
    CloseBracketNotFound,
    ParseWeightError(ParseIntError),
    NameNotFound,
}

impl std::fmt::Display for ProgramCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramCreationError::OpenBracketNotFound => {
                write!(f, "open bracket is required to identify weight")
            }
            ProgramCreationError::CloseBracketNotFound => {
                write!(f, "close bracket is required to identify weight")
            }
            ProgramCreationError::ParseWeightError(e) => {
                write!(f, "unable to parse given weight value into usize: {}", e)
            }
            ProgramCreationError::NameNotFound => {
                write!(f, "name str not found before open bracket")
            }
        }
    }
}

impl Error for ProgramCreationError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        "error creating program"
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::str::FromStr for Program {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obp = s
            .find("(")
            .ok_or_else(|| ProgramCreationError::OpenBracketNotFound)?;

        let cbp = s
            .find(")")
            .ok_or_else(|| ProgramCreationError::CloseBracketNotFound)?;

        let name: String = s
            .chars()
            .take(obp - 1)
            .filter(|c| !c.is_whitespace())
            .collect();

        if name.is_empty() {
            return Err(Box::new(ProgramCreationError::NameNotFound));
        }

        let weight: usize = s[obp + 1..cbp]
            .parse()
            .map_err(|e| ProgramCreationError::ParseWeightError(e))?;

        let mut children: Vec<String> = Vec::new();
        if let Some((_, children_str)) = s.split_once("->") {
            children.extend(
                children_str
                    .split(',')
                    .map(|child_str| child_str.trim().to_string()),
            );
        }

        Ok(Self {
            name,
            weight,
            children,
        })
    }
}

trait FindRoot {
    fn find_root(&self) -> Option<&Program>;
}
impl FindRoot for Box<[Program]> {
    fn find_root(&self) -> Option<&Program> {
        let children: Vec<&String> = self
            .iter()
            .filter(|x| !x.children.is_empty())
            .flat_map(|x| x.children.iter().collect::<Vec<&String>>())
            .collect();

        self.iter().find(|x| !children.contains(&&x.name))
    }
}

#[allow(dead_code)]
pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let programs: Box<[Program]> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|x| Program::from_str(x))
        .collect::<Result<Box<[Program]>, _>>()?;

    println!("{:#?}", programs.find_root());

    Ok(())
}

// good structure so far
// now build programs to be a HashMap<String, ProgramContent>
// for faster lookups

#[allow(dead_code)]
pub fn part_two() {}
