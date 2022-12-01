mod ast;
mod eval;
mod parse;
use ast::*;
use parse::*;
use pest::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("cannot read file");
    match LamParser::parse(Rule::prog, &file) {
        Ok(mut pairs) => {
            let tm = parse_term(pairs.next().unwrap().into_inner());
            println!("term  : {:?}", tm);
            println!("value : {:?}", tm.run());
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e)
        }
    }
}
