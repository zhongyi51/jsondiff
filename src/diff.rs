use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::thread;
use std::time::SystemTime;
use serde_json::Value;
use crate::{DiffCommand, pretty_print_diff};
use crate::jsonpath::{JsonPath, PathIndex};

#[derive(Debug, Clone)]
pub struct JsonDiff<'a> {
    left: Option<&'a Value>,
    right: Option<&'a Value>,
}

impl<'a> JsonDiff<'a> {
    fn new() -> JsonDiff<'a> {
        JsonDiff {
            left: None,
            right: None,
        }
    }
    fn set_left(&mut self, left: &'a Value) {
        self.left = Some(left);
    }
    fn set_right(&mut self, right: &'a Value) {
        self.right = Some(right);
    }
    fn is_diff(&self, key_only: bool) -> bool {
        if key_only {
            self.left.is_none() && self.right.is_some() || self.left.is_some() && self.right.is_none()
        } else {
            self.left != self.right
        }
    }
    pub fn get_left(&self) -> Option<&'a Value> {
        self.left
    }
    pub fn get_right(&self) -> Option<&'a Value> {
        self.right
    }
}

pub fn diff_json<'a>(left: &'a Value, right: &'a Value, command: &DiffCommand) {
    let (left_map, right_map) =
        if command.threading {
            thread::scope(|scope| {
                let right_map_join = scope.spawn(|| extract_json_pair(left, JsonPath::from_str("$").unwrap()));
                let left_map_join = scope.spawn(|| extract_json_pair(right, JsonPath::from_str("$").unwrap()));
                (right_map_join.join().unwrap(), left_map_join.join().unwrap())
            })
        } else {
            (extract_json_pair(left, JsonPath::from_str("$").unwrap()),
             extract_json_pair(right, JsonPath::from_str("$").unwrap()))
        };

    let mut diff_map = HashMap::new();
    for (k, v) in &left_map {
        diff_map.entry(k).or_insert(JsonDiff::new()).set_left(v);
    }
    for (k, v) in &right_map {
        diff_map.entry(k).or_insert(JsonDiff::new()).set_right(v);
    }
    let collect_res = diff_map.into_iter()
        .filter(|(k, _)| !command.ignore_case.iter().any(|ignore_k| ignore_k.can_ignore(k)))
        .filter(|(k, v)| v.is_diff(command.key_only))
        .collect();
    pretty_print_diff(&collect_res);
}

pub fn extract_json_pair(json: &Value, path: JsonPath) -> Vec<(JsonPath, &Value)> {
    let mut json_pair = Vec::new();
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) | Value::Number(_) => {
            json_pair.push((path.clone(), json));
        }
        Value::Array(json_vec) => {
            for (i, sub_json) in json_vec.iter().enumerate() {
                let mut sub_path = path.clone();
                sub_path.extend(PathIndex::Index(i));
                json_pair.extend(extract_json_pair(sub_json, sub_path));
            }
        }
        Value::Object(json_map) => {
            for (k, sub_json) in json_map.iter() {
                let mut sub_path = path.clone();
                sub_path.extend(PathIndex::Key(k.clone()));
                json_pair.extend(extract_json_pair(sub_json, sub_path));
            }
        }
    }
    return json_pair;
}

#[cfg(test)]
mod diff_test {
    use std::str::FromStr;
    use serde_json::Value;
    use crate::diff::extract_json_pair;
    use crate::jsonpath::JsonPath;

    #[test]
    fn test_extract() {
        assert!(vec!["a", "b"] == vec!["a", "b"]);
        let raw_json = r#"{ "glossary": { "title": "example glossary", "GlossDiv": { "title": "S", "GlossList": { "GlossEntry": { "ID": "SGML", "SortAs": "SGML", "GlossTerm": "Standard Generalized Markup Language", "Acronym": "SGML", "Abbrev": "ISO 8879:1986", "GlossDef": { "para": "A meta-markup language, used to create markup languages such as DocBook.", "GlossSeeAlso": ["GML", "XML"] }, "GlossSee": "markup" } } } } }"#;
        let json: Value = serde_json::from_str(raw_json).unwrap();
        let res = extract_json_pair(&json, JsonPath::from_str("$").unwrap());
        println!("res {:?}", res);
    }
}