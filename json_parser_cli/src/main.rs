use std::{
    env,
    fs::{self},
};

use bumpalo::Bump;
use json_parser::parser::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let Some(path) = env::args().nth(1) else {
        panic!("missing path")
    };

    let input = fs::read_to_string(path).unwrap();

    let bump = Bump::new();
    let parser = Parser::new(&input);

    let parsed = parser.parse(&bump)?;

    println!("{:#?}", parsed.flattened());

    Ok(())
}
