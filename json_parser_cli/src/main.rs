use std::{
    env,
    fs::{self},
};

use bumpalo::Bump;
use json_parser::parser::Parser;

fn main() {
    let Some(path) = env::args().nth(1) else {
        panic!("missing path")
    };

    let input = fs::read_to_string(path).unwrap();

    let bump = Bump::new();
    let parser = Parser::new(&input);

    match parser.parse(&bump) {
        Ok(res) => println!("{:#?}", res.flattened()),
        Err(e) => eprintln!("{e}"),
    }
}
