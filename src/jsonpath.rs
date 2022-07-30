use std::hash::{Hash, Hasher};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VALID_REGEX:Regex=Regex::new(r"^\*|\[(0|[1-9][0-9]*)\]|\p{L}+$").unwrap();
}

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct JsonPath(Vec<String>);

impl JsonPath{
    pub fn new()->JsonPath{
        return JsonPath(vec![]);
    }
    pub fn extend(&mut self,exp:&str)->Result<(),()>{
        if VALID_REGEX.is_match(exp){
            Ok(self.0.push(exp.into()))
        }else{
            Err(())
        }
    }
    pub fn show(&self)->String{
        self.0.join(".")
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
            if !VALID_REGEX.is_match(sub_exp) && i!=0{
                return Err(format!("Invalid sub-expression:{}",sub_exp));
            }
            sub_exp_vec.push(sub_exp.into());
        }
        return Ok(JsonPath(sub_exp_vec));
    }
}