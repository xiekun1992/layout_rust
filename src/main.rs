use engine::parser::Parser;
use std::fs;
// use std::path::Path;

mod engine;

fn main() {
    // let path = Path::new("./t1.html");
    let content = fs::read_to_string("./t1.html").unwrap();

    let mut parser = Parser::new();
    parser.parse(content);
}
