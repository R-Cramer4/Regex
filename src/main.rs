#[path = "regex/parser.rs"]
mod parser;
#[path = "regex/compiler.rs"]
mod compiler;
#[path = "regex/matcher.rs"]
mod matcher;


fn main() {
    let regex = parser::create("a(b)c+".to_string());
    //regex.print(&"".to_string());

    let fsm = compiler::compile(regex);
    //fsm.print();

    let matches = matcher::match_string(&fsm, "ubdisafbiabcccccoisnadfiuabcbbccabc".to_string());

    match matches{
        Some(a) => for i in a { i.print(None) },
        None => println!("Nothing matched"),
    };
}
