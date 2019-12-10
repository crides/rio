mod ast;
use ast::{
    type_def,
};

fn main() {
    let test_str = include_str!("test.rio");
    println!("{:#?}", type_def(test_str));
}
