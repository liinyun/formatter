use tree_sitter::Parser;
use std::fs::File;

fn main() {
    let code = r#"
function init(a,b) {
  var c = 2;
	MenuInputDelegate.initializ();
}
"#;
    let mut parser = Parser::new();
    let language = tree_sitter_monkeyc::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Monkeyc parser");
    let tree = parser.parse(code, None).unwrap();
    let file = File::create("tree.txt").expect("Failing to create file");
    // let file = File::create("tee.dot");
    tree.print_dot_graph(&file);
}
