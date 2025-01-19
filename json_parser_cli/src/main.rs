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
    // let mut file = file.as_bytes().to_vec();

    // let root = simd_json::to_borrowed_value(&mut file);
    // dbg!(&root);

    let bump = Bump::new();
    let parser = Parser::new(&input);
    parser.parse(&bump);
}
