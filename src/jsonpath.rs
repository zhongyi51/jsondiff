use std::cmp::min;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

lazy_static! {
    static ref VALID_NON_KEY_REGEX:Regex=Regex::new(r"^\*|\[(0|[1-9][0-9]*)\]$").unwrap();
    static ref IGNORE_CASE_ESCAPE:Regex=Regex::new(r"\\(?P<x>.)").unwrap();
    static ref IGNORE_CASE_PATH_TYPE:Regex=Regex::new("\\.\"(.+?)\"|\\.\\[([0-9]+?)\\]|\\.(\\*)|\\.<(.+?)>").unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PathIndex {
    Index(usize),
    Key(String),
    Any,
    Regex(String),
}

impl ToString for PathIndex {
    fn to_string(&self) -> String {
        match self {
            PathIndex::Index(i) => i.to_string(),
            PathIndex::Key(k) => k.clone(),
            PathIndex::Any => "*".into(),
            PathIndex::Regex(r) => r.to_string()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JsonPath(Vec<PathIndex>);

impl JsonPath {
    pub fn new() -> JsonPath {
        return JsonPath(vec![]);
    }
    pub fn extend(&mut self, exp: PathIndex) {
        self.0.push(exp);
    }
    pub fn show(&self) -> String {
        self.0.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>().join(".")
    }
    pub fn can_ignore(&self, child: &JsonPath) -> bool {
        if self.0 == child.0 {
            return true;
        }
        for pair in self.0.iter().zip_longest(child.0.iter()) {
            match (pair.as_ref().left(), pair.as_ref().right()) {
                (Some(PathIndex::Any), _) => return true,
                (Some(PathIndex::Regex(re)), Some(PathIndex::Key(k))) => {
                    if !Regex::new(re).unwrap().is_match(k) {
                        return false;
                    }
                }
                (Some(x), Some(y)) if x != y => return false,
                _ => (),
            }
        }
        return true;
    }
}

impl FromStr for JsonPath {
    type Err = String;

    fn from_str(exp: &str) -> Result<Self, Self::Err> {
        if !exp.starts_with("$") {
            return Err("Invalid expression! Expression must start with '$'".into());
        }

        let mut sub_exp_vec = Vec::new();

        for (i, c) in IGNORE_CASE_PATH_TYPE.captures_iter(exp).enumerate() {
            if let Some(key) = c.get(1) {
                sub_exp_vec.push(PathIndex::Key(key.as_str().to_string()));
            } else if let Some(index) = c.get(2) {
                sub_exp_vec.push(PathIndex::Index(usize::from_str(index.as_str()).unwrap()));
            } else if let Some(_) = c.get(3) {
                sub_exp_vec.push(PathIndex::Any);
            } else if let Some(r) = c.get(4) {
                if let Ok(re) = Regex::new(r.as_str()) {
                    sub_exp_vec.push(PathIndex::Regex(r.as_str().to_string()));
                }
            } else {
                panic!("Invalid path content at ignore case index: {}", i);
            }
        }
        return Ok(JsonPath(sub_exp_vec));
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;
    use crate::jsonpath::IGNORE_CASE_PATH_TYPE;

    #[test]
    fn test_regex() {
        let ss = "$.\"ss\".[3].\"[4]\".\"5\".\"hh\".*.<[0-9][a-z]>";
        for l in IGNORE_CASE_PATH_TYPE.captures_iter(ss) {
            println!("{:?}", l);
        }
    }
}