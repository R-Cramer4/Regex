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
            0 => find_match(fsm, chs.collect(), 0),
            a => {
                chs.nth(a - 1);
                find_match(fsm, chs.collect(), a as i32)
            }
        };
        match g {
            Some(mut a) => {
                groups.append(&mut a);
            },
            None => (),
        }
    }
    match groups.is_empty(){
        true => None,
        false => Some(groups),
    }
}

fn find_match(fsm: &compiler::FSM, str: String, start: i32) -> Option<Vec<Group>>{
    //TODO implement groups
        
    // How to match
    // get list of all nodes we can get to from 0 characters
    // from all of those nodes, get list of all nodes that can be accessed with 1 char
    // repeat until all characters are consumed or no more characters can be consumed
    // 
    // returns all matches starting at this location
    //
    // have a list of nodes
    // pass along with the character
    // returns a new list of nodes
    // if one of the nodes is the end, match found
    // if length == 0, we are done and can return
    let mut strings: Vec<String> = vec![];

    let mut nodes: Vec<&compiler::Node> = vec![fsm.nodes.get(&0)?];
    let mut chs = str.chars();
    let mut string = "".to_string();

    while !nodes.is_empty() {
        let c = match chs.next(){
            Some(a) => a,
            None => break,
        };
        get_nodes(fsm, &mut nodes, c);

        // now need to check if one of the nodes is the last node
        for i in nodes.iter() {
            if i.edges.len() == 0 {
                // last node
                strings.push(string.clone());
            }
        }
        string.push(c);
    }
    // Here we have all the different strings that match our regex
    if strings.is_empty() {
        return None;
    }
    
    let mut groups: Vec<Group> = vec![];
    for i in strings.iter() {
        groups.push(Group{
            str: i.to_string(),
            loc: start,
            groups: None, // TODO implement
        });
    }
    Some(groups)
}
fn get_nodes<'a>(fsm: &'a compiler::FSM, nodes: &mut Vec<&'a compiler::Node>, c: char){
    // go through nodes until consumes a character
    // then we can add that node to the consumption list

    let mut q: VecDeque<&compiler::Node> = VecDeque::new();
    for i in nodes.iter() {
        q.push_back(i);
    }
    nodes.clear();
    // will add nodes to n when we reach them

    // q has all the nodes we started with
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        // loop through n's edges and get all next nodes
        for i in n.edges.iter() {
            let e = match fsm.edges.get(*i){
                Some(a) => a,
                None => break,
            };
            if e.taken.is_some_and(|x| x) {
                continue; // cant take this rule
            }
            match e.rule.rule.as_str() {
                "" => {
                    // is an epsilon, we can't stop here, need to consume a character
                    q.push_back(fsm.nodes.get(&e.n2).unwrap());
                    if e.n2 == (fsm.nodes.len() - 1) as i32 {
                        nodes.push(fsm.nodes.get(&e.n2).unwrap());
                    }
                }
                a => {
                    if a.contains(c) {
                        // can reach this node and have consumed 1 char so we add to final list
                        nodes.push(fsm.nodes.get(&e.n2).unwrap());
                    }
                }
            }
        }
    }
}
