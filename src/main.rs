mod ast;
mod eval;
mod parse;
use ast::*;
use parse::*;
use pest::Parser;
use std::fs;

fn main() {
    let file = fs::read_to_string("test.txt").expect("cannot read file");
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
