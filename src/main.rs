use rustpython_parser::{ast, Parse};

fn main() {
    let python_source = r#"
mutez = int
def is_odd(i: mutez) -> bool:
  return bool(i & 1)
"#;
    let ast = ast::Suite::parse(python_source, "<embedded>");

    dbg!("{}", ast.unwrap());
}
