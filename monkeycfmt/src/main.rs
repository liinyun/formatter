use tree_sitter::{InputEdit, Language, Parser, Point};

fn main() {
    let code = r#"
"#;
    let mut parser = Parser::new();
    let language = tree_sitter_monkeyc::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Monkeyc parser");
    let tree = parser.parse(code, None).unwrap();

    println!("Hello, world!");
}
