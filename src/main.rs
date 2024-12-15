#[path = "regex/parser.rs"]
mod parser;
#[path = "regex/compiler.rs"]
mod compiler;
#[path = "regex/matcher.rs"]
mod matcher;


fn main() {
    let regex = parser::create("a(b)c+".to_string());
    regex.print(&"".to_string());

    let fsm = compiler::compile(regex);
    
    fsm.print();
}
