use std::fs::File;
use std::io::Read;
use clap::{Parser};
use serde_json::{Value};
use crate::jsonpath::JsonPath;
use anyhow::Result;

#[derive(Parser, Debug, Clone)]
#[clap(name = "json-diff", version = "0.0.1-alpha")]
pub struct DiffCommand {
    #[clap(short, long, action)]
    pub key_only: bool,

    #[clap(short, long, action)]
    pub replace_backslash: bool,

    #[clap(short, long, value_parser)]
    pub ignore_case: Vec<JsonPath>,

    #[clap(value_parser = json_src_parser)]
    pub json1: JsonSrc,

    #[clap(value_parser = json_src_parser)]
    pub json2: JsonSrc,

}

#[derive(Debug, Clone)]
pub enum JsonSrc {
    File(String),
    Json(String),
}

impl JsonSrc {
    pub fn get_json(&self) -> Result<Value> {
        return match self {
            JsonSrc::File(path_dir) => {
                let mut fd = File::open(path_dir).unwrap();
                let mut buf = String::new();
                fd.read_to_string(&mut buf)?;
                Ok(serde_json::from_str(&buf)?)
            }
            JsonSrc::Json(s) => {
                Ok(serde_json::from_str(s)?)
            }
        };
    }
}


fn json_src_parser(json: &str) -> Result<JsonSrc, String> {
    if let Ok(file) = File::open(json) {
        return Ok(JsonSrc::File(json.into()));
    }
    return Ok(JsonSrc::Json(json.into()));
}
