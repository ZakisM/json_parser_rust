use std::{fs::File, io::Read};

use bumpalo::Bump;
use json_parser::parser::Parser;

fn main() {
    let mut file = File::open("./test_data/mesh.json").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    // let root = simd_json::to_borrowed_value(&mut bytes).unwrap();
    // dbg!(&root);

    let parser = Parser::new(&bytes);
    let bump = Bump::new();
    let root = parser.parse(&bump).unwrap();

    // let res = root.as_flattened();

    // dbg!(&res);
}
