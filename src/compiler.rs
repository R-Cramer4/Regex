use std::rc::Rc;

use crate::parser;

pub struct FSM{
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
struct Node{
    num: i32,
}
struct Edge{
    n1: Rc<Node>,
    n2: Rc<Node>,
    rule: Rule
}
struct Rule{
    // either an epsilon, which doesnt consume a character
    // something that consumes a character

    rule: String,
    group: bool, // if in a group, it will return the character upon consumption
}

pub fn compile(ast: parser::Expression) -> FSM{
    println!("{}", ast.get_tree());
    let mut i = 0; // counts nodes
    let mut fsm = FSM{
        nodes: vec![Node{num: i}], 
        edges: vec![], 
    };
    i += 1;


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

impl FSM{
    pub fn print(&self){
        for i in self.nodes.iter() {
            println!("{}", i.num);
        }
        for i in self.edges.iter() {
            println!("{} - {}, {}", i.n1.num, i.n2.num, i.rule.rule);
        }
    }
}
