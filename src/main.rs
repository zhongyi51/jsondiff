use std::time::SystemTime;
use crate::command::DiffCommand;
use clap::Parser;
use crate::diff::diff_json;
use crate::print::pretty_print_diff;

mod command;
mod jsonpath;
mod diff;
mod print;

fn main() {
   let cmd=DiffCommand::parse();
    run_cmd(cmd);
}

fn run_cmd(cmd:DiffCommand){
    let t0=SystemTime::now();
    let left=cmd.json1.get_json().expect("Invalid json1");
    let right=cmd.json2.get_json().expect("Invalid json2");


    diff_json(&left,&right,&cmd);
    println!("Finish comparison in {} ms.",t0.elapsed().unwrap().as_micros() as f64 / 1000.0)
}