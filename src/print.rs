use std::collections::HashMap;
use colored::Colorize;
use serde_json::Value;
use crate::diff::JsonDiff;
use crate::jsonpath::JsonPath;

pub fn pretty_print_diff(diff_map:&HashMap<&JsonPath,JsonDiff>){
    if diff_map.len()==0 {
        println!("Two jsons are {}.","same".blue());
    }else {
        println!("There are {} diff:\n",diff_map.len().to_string().blue());
    }
    diff_map.iter()
        .map(|(k,v)|{
            pretty_format_diff(k.show(),&v.get_left(),&v.get_right())
        })
        .for_each(|s|println!("{}",s));
}

fn pretty_format_diff(path:String,left:&Option<&Value>,right:&Option<&Value>)->String{
    let (t1,v1)=get_type_and_value(left);
    let (t2,v2)=get_type_and_value(right);
    format!("{}:\n\
            \t left({}):\n\
            \t \t {}\n\
            \t right({}):\n\
            \t \t {}\n",path,t1.blue(),v1.to_string().green(),t2.blue(),v2.to_string().red())
}



fn get_type_and_value<'a>(value:&'a Option<&Value>)-> (&'static str, Box<dyn ToString+'a>) {
    match value{
        None => {("None",Box::new(""))}
        Some(Value::Null) => {("Json:Null",Box::new("Null"))}
        Some(Value::String(s))=>{("Json:String",Box::new(s))}
        Some(Value::Number(n))=>{("Json:Number",Box::new(n))}
        Some(Value::Array(_))=>{("Json:Array",Box::new("[...]"))}
        Some(Value::Object(_))=>{("Json:Object",Box::new("{...}"))}
        Some(Value::Bool(b))=>{("Json:Bool",Box::new(b))}
    }
}