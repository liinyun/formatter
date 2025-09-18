use tree_sitter::{InputEdit, Language, Parser, Point};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_monkeyc::LANGUAGE.into())
        .expect("Ero loading Rust programmar");
    println!("Hello, world!");
}
