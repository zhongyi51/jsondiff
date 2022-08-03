use std::hash::{Hash, Hasher};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

lazy_static! {
    static ref VALID_NON_KEY_REGEX:Regex=Regex::new(r"^\*|\[(0|[1-9][0-9]*)\]|\p{L}+$").unwrap();
}

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum PathIndex{
    Index(usize),
    Key(String),
    Any,
}

impl ToString for PathIndex{
    fn to_string(&self) -> String {
        match self {
            PathIndex::Index(i) => i.to_string(),
            PathIndex::Key(k) => k.clone(),
            PathIndex::Any => "*".into(),
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct JsonPath(Vec<PathIndex>);

impl JsonPath{
    pub fn new()->JsonPath{
        return JsonPath(vec![]);
    }
    pub fn extend(&mut self,exp:PathIndex){
        self.0.push(exp);
    }
    pub fn show(&self)->String{
        self.0.iter()
            .map(|c|c.to_string())
            .collect::<Vec<String>>().join(".")
    }
}

impl FromStr for JsonPath{
    type Err = String;

    fn from_str(exp: &str) -> Result<Self, Self::Err> {

        if !exp.starts_with("$") {
            return Err("Invalid expression! Expression must start with '$'".into());
        }

        let mut sub_exp_vec =Vec::new();
        for (i,sub_exp) in exp.split(".").enumerate(){
            if !VALID_NON_KEY_REGEX.is_match(sub_exp) && i!=0{
                sub_exp_vec.push(PathIndex::Key(sub_exp.into()));
            }else if sub_exp=="*"{
                sub_exp_vec.push(PathIndex::Any);
            }else if let Ok(i)=usize::from_str(sub_exp){
                sub_exp_vec.push(PathIndex::Index(i));
            }
        }
        return Ok(JsonPath(sub_exp_vec));
    }
}