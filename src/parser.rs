use std::char;

enum Parts {
    Character(Character),
    Group(Group),
    Other,
}
struct Character{
    content: Option<char>,
}
struct Modifier{
    content: Option<char>,
}
struct Group{
    content: Box<Option<Expression>>,
}
struct Term{
    content: Box<Option<Parts>>,
    next: Option<Modifier>,
}
pub struct Expression{
    content: Box<Option<Term>>,
    next: Box<Option<Expression>>,
}
// Expression := Term Expression | None
// Term := Group Modifier | None
// Term := Character Modifier | None
//
// Term := Parts Modifier | None
//
//
// TODO things to implemet
// |
// \
// \special chars like \d
// non capturing groups
// character classes


impl Parts{
    fn parse(&mut self, str: String) -> String{
        let first = str.chars().next();
        match self{
            Self::Character(a) =>{
                a.character(str)
            }
            Self::Group(a) => {
                a.group(str)
            }
            _ => {
                // nothing, try to parse highest level
                // if (, parse as group
                // else parse as character
                match first{
                    Some('(') => {
                        *self = Self::Group(Group{content: Box::new(None)});
                        self.parse(str)
                    },
                    Some(_) => {
                        *self = Self::Character(Character{content: None});
                        self.parse(str)
                    }
                    None => {
                        println!("No character");
                        "".to_string()
                    }
                }
            }
        }
    }
    fn print(&self){
        match self{
            Self::Character(a) => {
                println!("{}", a.content.unwrap());
            },
            Self::Group(a) => {
                a.print();
            },
            _ =>{
                ()
            }
        }
    }
    fn get_tree(&self) -> String{
        match self{
            Self::Character(a) => {
                a.content.unwrap().to_string()
            }
            Self::Group(a) => {
                (*a.content).as_ref().unwrap().get_tree()
            }
            _ => {
                "Other".to_string()
            }
        }
    }
}

impl Character {
    fn character(&mut self, str: String) -> String{
        let mut chars = str.chars();
        let first = chars.next().unwrap();
        if first.is_alphabetic() {
            self.content = Some(first);
        }
        
        let leftovers: String = chars.collect();
        return leftovers.to_string();
    }
}
impl Modifier {
    fn modifier(&mut self, str: String) -> String{
        let mut chars = str.chars();
        let first = str.chars().nth(0).unwrap();
        let leftovers: String;
        match first{
            '+' | '*' | '?' => {
                self.content = Some(chars.next().unwrap());
                leftovers = chars.collect();
            }
            _ => {
                self.content = None;
                leftovers = chars.collect();
            }
        }
        return leftovers;
    }
    fn print(&self){
        match self.content {
            Some(a) => {
                println!("{}", a);
            },
            None =>{
                ()
            }
        }
    }
}
impl Group{
    fn group(&mut self, str: String) -> String{
        let mut leftovers = str.chars().rev();
        let mut ch = leftovers.next().unwrap();
        let mut group: Vec<char> = vec![];
        while ch != ')'{
            match ch{
                a =>{
                    group.push(a);
                }
            }
            ch = leftovers.next().unwrap();
        }
        let mut new_leftovers = leftovers.rev();
        new_leftovers.next(); // gets rid of the first (
        let new_str: String = new_leftovers.collect();

        match &mut *self.content{
            Some(a) => {
                a.expression(new_str);
            }
            None => {
                self.content = Box::new(Some(Expression::new()));
                self.content.as_mut().as_mut().unwrap().expression(new_str);
            }
        }
        group.into_iter().rev().collect()
    }
    fn print(&self){
        match self.content.as_ref(){
            Some(a) =>{
                println!("(");
                a.print();
                println!(")");
            }
            None => ()
        }
    }
}
impl Term{
    // returns any leftovers
    fn parse(&mut self, str: String) -> String{
        let mut leftovers;
        match &mut *self.content{
            Some(a) => {
                leftovers = a.parse(str);
            }
            None => {
                self.content = Box::new(Some(Parts::Other));
                leftovers = self.parse(str);
            }
        }
        match leftovers.as_str(){
            "" => {
                "".to_string()
            }
            _ => {
                // try to parse modifier
                self.next = Some(Modifier{content: None});
                leftovers = self.next.as_mut().unwrap().modifier(leftovers);
                leftovers
            }
        }
    }
    fn print(&self){
        match self.content.as_ref().as_ref(){
            Some(a) => {
                a.print();
            },
            _ => ()

        }
        match &self.next{
            Some(a) => {
                a.print();
            },
            _ => ()
        }
    }
    fn get_tree(&self) -> String{
        let mut str = (*self.content).as_ref().unwrap().get_tree().to_string();
        match &self.next{
            Some(a) => {
                match a.content{
                    Some(a) => str.push(a),
                    None => ()
                }
            }
            None =>{()}
        }
        str
    }
}

impl Expression {
    pub fn new() -> Expression{
        Expression {content: Box::new(None), next: Box::new(None)}
    }
    pub fn expression(&mut self, str: String){
        *self.content = Some(Term{content: Box::new(None), next: None});
        let leftovers: String = self.content.as_mut().as_mut().unwrap().parse(str);
        if leftovers != "" {
            self.next = Box::new(Some(Expression::new()));
            self.next.as_mut().as_mut().unwrap().expression(leftovers);
        }
    }
    pub fn print(&self){
        self.content.as_ref().as_ref().unwrap().print();
        match self.next.as_ref(){
            Some(exp) => exp.print(),
            None => return,
        }
    }
    pub fn get_tree(&self) -> String{
        let mut str = (*self.content).as_ref().unwrap().get_tree();
        match self.next.as_ref(){
            Some(a) => {
                str.push_str(" - ");
                str.push_str(a.get_tree().as_str())
            }
            None => {()}
        }

        return str;
    }
}
