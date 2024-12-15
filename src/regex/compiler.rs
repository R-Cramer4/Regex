use std::collections::BTreeMap;

use crate::parser;
use crate::parser::Parts;


pub struct FSM{
    nodes: BTreeMap<i32, Node>,
    edges: Vec<Edge>,
}
struct Node{
    group: i32,
}
struct Edge{
    n1: i32, // index in the vec
    n2: i32,
    rule: Rule
}
struct Rule{
    // either an epsilon, which doesnt consume a character
    // something that consumes a character

    rule: String,
    group: i32, // if in a group, it will return the character upon consumption
}

pub fn compile(ast: parser::Regex) -> FSM{
    let mut i = 0; // counts nodes
    let mut fsm = FSM{
        nodes: BTreeMap::new(),
        edges: vec![],
    };
    i += 1;

    generate_fsm(&mut fsm, Parts::Regex(ast), &mut i, 0);

    // how to build the finite state machine
    // enter expression
    //  enter term
    //      if character, add node with one edge leading to it that consumes the char
    //      if modifier, add epsilons
    // create node and add epsilon to it from end to first node of next expression
    // enter next expression
    // (ignore groups for now)


    fsm
}
fn generate_fsm(fsm: &mut FSM, p: Parts, i: &mut i32, group: i32){
    match p {
        Parts::Expression(a) =>{
        }
        Parts::Term(a) => {
        }
        Parts::Group(a) => {
        }
        Parts::Modifier(a) => {
            // actually do something
        }
        Parts::Character(a) => {
            // consumes a character, just moves to the next node
        }
        Parts::Other => {
            println!("Not supposed to be here");
        }
        _ => (),
    }
}

impl FSM{
    pub fn print(&self){
        for i in self.nodes.iter() {
            println!("{}", i.1.group);
        }
        for i in self.edges.iter() {
            println!("{} - {}, {}", i.n1, i.n2, i.rule.rule);
        }
    }
}
