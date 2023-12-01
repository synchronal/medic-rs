// @related [tests](medic-src/src/outdated_check/outdated_test.rs)

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Outdated {
    pub deps: Vec<OutdatedDep>,
    pub remedy: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct OutdatedDep {
    pub latest: String,
    pub name: String,
    pub parent: Option<String>,
    pub version: String,
}

#[derive(Debug)]
pub struct ParseError;

impl Outdated {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut deps: Vec<OutdatedDep> = vec![];
        let mut remedy: Option<String> = None;

        s.lines().for_each(|l| {
            let mut split = l.split("::");
            split.next();
            match split.next() {
                Some("remedy") => {
                    remedy = Some(split.next().unwrap().to_owned());
                }
                Some("outdated") => {
                    let outdated = split.collect::<Vec<&str>>().join("::");
                    let dep = OutdatedDep::from_str(&outdated).unwrap();
                    deps.push(dep);
                }
                Some(_) => {}
                None => {}
            }
        });

        Ok(Self { deps, remedy })
    }
}

impl OutdatedDep {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut inputs: HashMap<&str, &str> = HashMap::with_capacity(4);

        for value_pair in s.trim().split("::") {
            if let Some((key, value)) = value_pair.split_once('=') {
                inputs.insert(key, value);
            } else {
                return Err(ParseError);
            }
        }

        let mut keys: Vec<&&str> = inputs.keys().collect();
        keys.sort();
        if keys == vec![&"latest", &"name", &"version"] {
            Ok(Self {
                latest: inputs.get("latest").unwrap().to_string(),
                name: inputs.get("name").unwrap().to_string(),
                parent: None,
                version: inputs.get("version").unwrap().to_string(),
            })
        } else if keys == vec![&"latest", &"name", &"parent", &"version"] {
            Ok(Self {
                latest: inputs.get("latest").unwrap().to_string(),
                name: inputs.get("name").unwrap().to_string(),
                parent: Some(inputs.get("parent").unwrap().to_string()),
                version: inputs.get("version").unwrap().to_string(),
            })
        } else {
            Err(ParseError)
        }
    }
}

impl std::str::FromStr for Outdated {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl std::str::FromStr for OutdatedDep {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
