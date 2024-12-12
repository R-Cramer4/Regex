pub mod parser;
pub mod compiler;


fn main() {
    let str = "a(b)c+";
    let ast = parse(str.to_string());

    let fsm = compiler::compile(ast);
    fsm.print();
}

fn parse(str: String) -> parser::Expression{
    let mut ast: parser::Expression = parser::Expression::new();
    ast.expression(str.to_string());
    ast
}
