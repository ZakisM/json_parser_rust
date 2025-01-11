use std::fs::{self};

use bumpalo::Bump;
use json_parser::parser::Parser;

fn main() {
    let file = fs::read_to_string("./test_data/photos.json").unwrap();
    // let mut file = file.as_bytes().to_vec();

    // let root = &simd_json::to_borrowed_value(&mut file);

    let parser = Parser::new(&file);
    let bump = Bump::new();

    if let Err(e) = parser.parse(&bump) {
        println!("{e}");
    };
}
