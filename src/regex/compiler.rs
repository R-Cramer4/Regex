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

pub fn compile(ast: parser::Expression) -> FSM{
    println!("{}", ast.get_tree());
    let mut i = 0; // counts nodes
    let mut fsm = FSM{
        nodes: BTreeMap::new(),
        edges: vec![],
    };
    i += 1;

    generate_fsm(&mut fsm, Parts::Expression(ast), &mut i, 0);

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
            generate_fsm(fsm, Parts::Term((*a.content).unwrap()), i, group);
            if (*a.next).is_some() {
                generate_fsm(fsm, Parts::Expression((*a.next).unwrap()), i, group);
            }
        }
        Parts::Term(a) => {
            generate_fsm(fsm, (*a.content).unwrap(), i, group);
            if a.next.is_some() {
                generate_fsm(fsm, Parts::Modifier(a.next.unwrap()), i, group);
            }
        }
        Parts::Group(a) => {
            generate_fsm(fsm, Parts::Expression((*a.content).unwrap()), i, group + 1);
        }
        Parts::Modifier(a) => {
            // actually do something
            let e = fsm.edges.last().unwrap();
            match a.content {
                // All of these assume it is just modifying a character
                Some('+') => {
                    // 1 or more
                    // create epsilon to reverse the last edge
                    let eps = Edge{n1: e.n2, n2: e.n1, rule: Rule{rule: "".to_string(), group}};
                    fsm.edges.push(eps);
                }
                Some('*') => {
                    let eps = Edge{n1: e.n2, n2: e.n1, rule: Rule{rule: "".to_string(), group}};
                    let eps2 = Edge{n1: e.n1, n2: e.n2, rule: Rule{rule: "".to_string(), group}};
                    fsm.edges.push(eps);
                    fsm.edges.push(eps2);
                    // 0 or more
                    // create epsilon along last edge, and epsilon to reverse the last edge
                }
                Some('?') => {
                    let eps = Edge{n1: e.n1, n2: e.n2, rule: Rule{rule: "".to_string(), group}};
                    fsm.edges.push(eps);
                    // 1 or none
                    // create epsilon along last edge
                }
                Some(_) => {
                    println!("Not implemented");
                }
                None => ()
            }
        }
        Parts::Character(a) => {
            // consumes a character, just moves to the next node
            let n = Node{group};
            fsm.nodes.insert(*i, n);
            fsm.edges.push(Edge{n1: *i - 1, n2: *i, rule: Rule{rule: a.content.unwrap().to_string(), group}});
            *i += 1;
        }
        Parts::Other => {
            println!("Not supposed to be here");
        }
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
