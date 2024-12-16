use std::collections::BTreeMap;

use crate::parser;
use crate::parser::Parts;


pub struct FSM{
    pub(crate) nodes: BTreeMap<i32, Node>,
    pub(crate) edges: Vec<Edge>,
}
pub(crate) struct Node{
    pub(crate) group: i32,
    pub(crate) edges: Vec<usize>, // points to index in edges
}
pub(crate) struct Edge{
    pub(crate) n1: i32, // index in the vec
    pub(crate) n2: i32,
    pub(crate) rule: Rule
}
pub(crate) struct Rule{
    // either an epsilon, which doesnt consume a character
    // something that consumes a character

    pub(crate) rule: String,
    pub(crate) group: i32, // if in a group, it will return the character upon consumption
}

pub fn compile(ast: parser::Regex) -> FSM{
    let mut fsm = FSM{
        nodes: BTreeMap::new(),
        edges: vec![],
    };
    fsm.nodes.insert(0, Node{group: 0, edges: vec![]});

    generate_fsm(&mut fsm, Parts::Regex(ast), 0, 0, None);
    fsm
}

// Takes the start node, returns an end node
fn generate_fsm(fsm: &mut FSM, p: Parts, group: i32, start: i32, end: Option<i32>) -> i32{
    match p {
        Parts:: Character(c) => {
            let i = fsm.nodes.len() as i32;
            fsm.nodes.insert(i, Node{group, edges: vec![]});
            fsm.edges.push(Edge{
                n1: start,
                n2: i,
                rule: Rule{
                    rule: c.content,
                    group
                }
            });
            fsm.nodes.get_mut(&start).unwrap().edges.push(fsm.edges.len() - 1);
            return i;
        }
        Parts:: Modifier(m) => {
            // takes the start and end of the thing t it is modifying
            // makes epsilons from start + m.start copies of t to m.end copies of t
            // if m.end == -1, makes epsilon from m.end to m.end - 1
            // TODO implement ranges, only works for +, *, and ?
            
            if m.end == -1 {
                fsm.edges.push(Edge{
                    n1: end.unwrap(),
                    n2: start,
                    rule: Rule{
                        rule: "".to_string(),
                        group
                    }
                });
                fsm.nodes.get_mut(&end.unwrap()).unwrap().edges.push(fsm.edges.len() - 1);
            }
            if m.start == 0 {
                fsm.edges.push(Edge{
                    n1: start,
                    n2: end.unwrap(),
                    rule: Rule{
                        rule: "".to_string(),
                        group
                    }
                });
                fsm.nodes.get_mut(&start).unwrap().edges.push(fsm.edges.len() - 1);
            }
            return end.unwrap();
            // to implememnt ranges
            // get all the nodes and edges that go between the start and end node
            // duplicate them m.start times
            // duplicate m.end - m.start more times
            // create epsilons from m.start + i to the end node to create the option to leave
        }
        Parts::Group(g) => {
            // TODO implement non capturing groups
            let ng = group + 1;
            let l = generate_fsm(fsm, Parts::Regex(g.content), ng, start, None);
            let i = fsm.nodes.len() as i32;
            fsm.nodes.insert(i, Node{group, edges: vec![]});
            fsm.edges.push(Edge{
                n1: l,
                n2: i,
                rule: Rule{
                    rule: "".to_string(),
                    group
                }
            });
            fsm.nodes.get_mut(&l).unwrap().edges.push(fsm.edges.len() - 1);
            return i;
        }
        Parts::Term(t) => {
            let mut l = generate_fsm(fsm, *t.content, group, start, None);
            // need to add the modifier
            if t.modif.is_some() {
                l = generate_fsm(fsm, Parts::Modifier(t.modif.unwrap()), group, start, Some(l));
            }
            return l;
        }
        Parts::Expression(e) => {
            let mut l = generate_fsm(fsm, Parts::Term(e.content), group, start, None);
            if e.next.as_ref().is_some(){
                l = generate_fsm(fsm, Parts::Expression(e.next.unwrap()), group, l, None);
            }
            return l;
        }
        Parts::Alternator(a) => {
            // generate between both sides and add an epsilon from the ends to the new end
            let l = generate_fsm(fsm, Parts::Expression(a.lhs), group, start, None);
            let mut r = -1;
            if a.rhs.is_some(){
                r = generate_fsm(fsm, Parts::Expression(a.rhs.unwrap()), group, start, None);
            }
            let i = fsm.nodes.len() as i32;
            fsm.nodes.insert(i, Node{group, edges: vec![]});
            fsm.edges.push(Edge{
                n1: l,
                n2: i,
                rule: Rule{
                    rule: "".to_string(),
                    group
                }
            });
            fsm.nodes.get_mut(&l).unwrap().edges.push(fsm.edges.len() - 1);
            if r != -1 {
                fsm.edges.push(Edge{
                    n1: r,
                    n2: i,
                    rule: Rule{
                        rule: "".to_string(),
                        group
                    }
                });
                fsm.nodes.get_mut(&r).unwrap().edges.push(fsm.edges.len() - 1);
            }
            return i;
        }
        Parts::Regex(r) => {
            return generate_fsm(fsm, Parts::Alternator(r.content), group, start, None);
        }
        Parts::Other => {
            println!("Not supposed to be here");
            0
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
