use std::collections::VecDeque;

use crate::compiler;

pub struct Group{
    // holds each time that the entire thing is matched
    // starting index
    // a list of groups
    str: String,
    loc: i32,
    groups: Option<Vec<Group>>,
}
impl Group {
    pub fn print(&self, offset: Option<String>){
        println!("{} at {}", self.str, self.loc);
        if self.groups.is_some() {
            let mut o = match offset{
                Some(a) => a,
                None => "".to_string(),
            };
            o.push('\t');
            for i in self.groups.as_ref().unwrap().iter() {
                i.print(Some(o.clone()));
            }
        }
    }
}

pub fn match_string(fsm: &compiler::FSM, str: String) -> Option<Vec<Group>>{
    let mut groups: Vec<Group> = vec![];
    if fsm.nodes.len() == 0 {
        return None
    }

    let max = str.chars().count();
    for i in 0..max {
        let mut chs = str.chars();
        let g = match i {
            0 => bfs(fsm, chs.collect()),
            a => {
                chs.nth(a - 1);
                bfs(fsm, chs.collect())
            }
        };
        match g {
            Some(a) => groups.push(a),
            None => (),
        }
    }
    match groups.is_empty(){
        true => None,
        false => Some(groups),
    }
}
fn bfs(fsm: &compiler::FSM, str: String) -> Option<Group>{
    // make a queue of nodes, this queue is the nodes reached on n characters consumed
    let mut q: VecDeque<&compiler::Node> = VecDeque::new();
    q.push_back(fsm.nodes.get(&0).unwrap());

    let mut chs = str.chars();
    while !q.is_empty() {
        // bfs
        let ch = chs.next();
        if ch.is_none() {
            return None;
        }
        let n = q.pop_front().unwrap();
        // TODO check if n is the end node, and if so we are done and can return
        let mut epsi_queue: VecDeque<&compiler::Node> = VecDeque::new();
        // n.edges has all the edges starting here
        for i in 0..n.edges.len() {
            let e = fsm.edges.get(i);
            if e.is_none() {
                println!("Searched for an edge that doesnt exist");
                return None;
            }
            let n2 = e.unwrap().n2;
            let r = &e.unwrap().rule;

            match r.rule.as_str() {
                "" => {
                    q.push_back(fsm.nodes.get(&n2).unwrap());

                    //TODO check if not already in queue
                    epsi_queue.push_back(fsm.nodes.get(&n2).unwrap());
                    
                    
                }
                a => {
                    if a.contains(ch.unwrap()) {
                        q.push_back(fsm.nodes.get(&n2).unwrap());
                    }
                }
            }
        }

    }

    None
}
