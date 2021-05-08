#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

mod env;
mod ast;

use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    input: String,
}

fn main() {
    let Opt{input} = Opt::from_args();
    let mut file = std::fs::File::open(&input).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    match grammar::SeqParser::new().parse(&source) {
        Ok(s) => {
            println!("{:#?}", s);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}