
pub(crate) enum Parts{
    Character(Character),
    Modifier(Modifier),
    Group(Group),
    Term(Term),
    Expression(Expression),
    Alternator(Alternator),
    Regex(Regex),
    Other,
}
pub(crate) struct Character{
    // can have 
    // characters: a, b, c, etc..
    // char types: \d for digit, \w for word character
    // character classes: [AEIOU] for one letter in the class
    pub(crate) content: String,
    pub(crate) not: bool,

}
pub(crate) struct Modifier{
    // can have
    // + : 1 or more
    // ? : 0 or 1
    // * : 0 or more
    // {x, y} : range between x and y times
    // ? : after a modifier, makes it lazy
    pub(crate) start: i32, // has to have at least this many
    pub(crate) end: i32, // -1 means it is just more
    pub(crate) greedy: bool, // stops as soon as possible

}
pub(crate) struct Group{
    // can have
    // ( ... ) : capturing
    // (?: ... ) : non capturing
    pub(crate) content: Regex,
    pub(crate) capturing: bool,
}
pub(crate) struct Term{
    // Group + Modifier | None
    // Character + Modifier | None
    pub(crate) content: Box<Parts>,
    pub(crate) modif: Option<Modifier>,
}
pub(crate) struct Expression{
    // Term + Expression | None
    pub(crate) content: Term,
    pub(crate) next: Box<Option<Expression>>,

}
pub(crate) struct Alternator{
    // lhs | rhs
    // checks if matches either
    // both are expressions
    pub(crate) lhs: Expression,
    pub(crate) rhs: Option<Expression>,
}
pub struct Regex{
    pub(crate) content: Alternator,
}
impl Character{
    fn new(str: String) -> (Character, Option<String>){
        let mut chs = str.chars();
        let next = chs.next();
        let content: String;
        let mut not = false;
        match next.unwrap(){
            '\\' => {
                // something escaped and special
                let t = chs.next().unwrap();
                match t {
                    'd' => content = "1234567890".to_string(), // it is a digit
                    'w' => {
                        content = "1234567890qwertyuiopasdfghjklzxcvbnm_".to_string(); // word char
                    }
                    's' => content = " \n\t\r".to_string(), // whitespace char
                    'D' => {
                        not = true;
                        content = "1234567890".to_string();
                    }
                    'W' => {
                        not = true;
                        content = "1234567890qwertyuiopasdfghjklzxcvbnm_".to_string(); // word char
                    }
                    'S' => {
                        not = true;
                        content = " \n\t\r".to_string(); // whitespace char
                    }
                    '.' | '*' | '?' | '+' | '\\' | '$' | '^' => content = t.to_string(),
                    _ => {
                        println!("Got {} and not an option", t);
                        content = "".to_string();
                    }
                                                                                    
                }
            }
            '[' => {
                // TODO Implement
                // capturing
                content = "".to_string();
            }
            a => {
                // normal
                content = a.to_string();
            }
        }
        let leftovers: String = chs.collect();

        (Character{
            content,
            not
        }, match leftovers.as_str() {
            "" => None,
            a => Some(a.to_string())
        })
    }
    fn print(&self, offset: &String){
        println!("{}char: {} {}", offset, self.content, self.not);
    }
}

impl Modifier{
    fn new(str: String) -> (Option<Modifier>, Option<String>){
        let mut chs = str.chars();
        let mut next = chs.next();
        let mut start = 0;
        let mut end = 1;
        let mut greedy = false;
        let mut modif = true;
        match next {
            Some('+') => {
                start = 1;
                end = -1;
            }
            Some('?') => (),
            Some('*') => end = -1,
            Some('{') => {
                // range
                // need to collect until the ','
                let mut first = vec![];
                next = chs.next();
                while next.unwrap() != ',' {
                    first.push(next.unwrap());
                    next = chs.next();
                }
                start = first.iter().collect::<String>().parse::<i32>().unwrap();
                let mut sec = vec![];
                next = chs.next();
                while next.unwrap() != '}' {
                    sec.push(next.unwrap());
                    next = chs.next();
                }
                let result = sec.iter().collect::<String>().parse::<i32>();
                match result {
                    Ok(a) => end = a,
                    Err(_) => end = -1,
                }
            }
            Some(a) => {
                println!("Got {} which isnt a modifier", a);
                modif = false;
            }
            None =>{
                println!("Tried to parse a modifier with nothing");
                modif = false;
            }
        }
        // need to check if greedy
        let mut leftovers: String = chs.collect();
        let mut lchs = leftovers.chars();
        match lchs.next() {
            Some('?') => {
                greedy = true;
                leftovers = lchs.collect();
            }
            Some(_) =>(),
            None => ()
        }
        if modif == false {
            leftovers = str;
        }
        (match modif{
            true => {
                Some(Modifier{
                    start,
                    end,
                    greedy,})
            }
            false => None,
        }, match leftovers.as_str(){
                "" => None,
                a => Some(a.to_string()),
        })
    }
    fn print(&self, offset: &String){
        println!("{}modifier: {}-{}, {}", offset, self.start, self.end, self.greedy);
    }
}

impl Group{
    fn new(str: String) -> Group{
        // parses an expression
        // doesnt have the closing bracket
        let mut capturing = true;
        let mut exp = str.chars();
        let content: String;
        match exp.next().unwrap(){
            '?' => {
                // non capturing group
                capturing = false;
                exp.next();
                // this gets rid of the "?:" that makes it not capturing
                content = exp.collect();

            }
            _ => {
                content = str;
            }
        }
        Group{
            content: create(content),
            capturing,
        }
    }
    fn print(&self, offset: &String){
        let ol = offset.as_str();
        let mut o = ol.to_string();
        println!("{} group, {}", offset, self.capturing);
        o.push('\t');
        self.content.print(&o)

    }
}

impl Term{
    fn new(str: String) -> (Term, Option<String>){
        println!("{}", str);
        // parse term
        // Term := Group | Character + Modifier | None
        let mut exp = str.chars();
        let mut temp = exp.next();
        let content: Parts;
        let leftovers: String;
        match temp {
            Some('(') => {
                let mut group_num = 1;
                let mut expres = vec![];
                let mut group_leftovers = vec![];
                let mut in_group = true;
                // parse group
                temp = exp.next();
                while temp.is_some(){
                    match temp.unwrap(){
                        '(' => {
                            group_num += 1;
                            expres.push('(');
                        }
                        ')' => {
                            group_num -= 1;
                            if group_num == 0 {
                                in_group = false;
                            }else{
                                expres.push(')');
                            }
                        }
                        a => {
                            if in_group {
                                expres.push(a);
                            }else {
                                group_leftovers.push(a);
                            }
                        }

                    }
                    temp = exp.next();
                }
                // have group and leftovers (That has modifier)
                content = Parts::Group(Group::new(expres.iter().collect()));
                leftovers = group_leftovers.iter().collect();
            }
            Some(_) => {
                // parse character
                let c = Character::new(str.clone());
                content = Parts::Character(c.0);
                leftovers = match c.1 {
                    Some(a) => a,
                    None => "".to_string(),
                }
            }
            None => {
                println!("Tried to parse a term with nothing");
                content = Parts::Other;
                leftovers = "".to_string();
            }
        }
        // leftovers has the modifier that I still need to parse
        let m = Modifier::new(leftovers);
        (Term{
            content: Box::new(content),
            modif: m.0
        }, m.1)

    }
    fn print(&self, offset: &String){
        println!("{} Term", offset);
        let ol = offset.as_str();
        let mut o = ol.to_string();
        o.push('\t');

        match self.content.as_ref(){
            Parts::Character(a) => {
                a.print(&o);
            }
            Parts::Group(a) => {
                a.print(&o);
            }
            _ =>(),
        }
        if self.modif.is_some() {
            self.modif.as_ref().unwrap().print(&o);
        }
    }
}

impl Expression {
    fn new(str: String) -> Expression {
        // parse expression and return
        let t = Term::new(str);
        Expression{
            content: t.0,
            next: match t.1{
                Some(a) => Box::new(Some(Expression::new(a))),
                None => Box::new(None),
            },
        }
    }
    fn print(&self, offset: &String){
        println!("{} Expression", offset);
        let ol = offset.as_str();
        let mut o = ol.to_string();
        o.push('\t');
        self.content.print(&o);
        if (*self.next).is_some() {
            (*self.next).as_ref().unwrap().print(&o);
        }
    }
}
impl Regex{
    pub fn print(&self, offset: &String){
        println!("{} Regex", offset);
        let ol = offset.as_str();
        let mut o = ol.to_string();
        o.push('\t');
        self.content.print(&o);
    }
}
impl Alternator{
    fn print(&self, offset: &String){
        println!("{} Alt", offset);
        let ol = offset.as_str();
        let mut o = ol.to_string();
        o.push('\t');
        self.lhs.print(&o);
        if self.rhs.is_some() {
            self.rhs.as_ref().unwrap().print(&o);
        }
    }
}

pub fn create(str: String) -> Regex{
    let mut regex = str.chars();
    let mut lhs = vec![];
    let mut rhs = vec![];
    let mut in_group = 0;
    let mut temp = regex.next();
    let mut right = false;
    while temp.is_some() {
        match temp.unwrap() {
            '(' => {
                in_group += 1;
                if !right {
                    lhs.push('(');
                }else{
                    rhs.push('(');
                }
            }
            ')' => {
                in_group -= 1;
                if !right {
                    lhs.push(')');
                }else{
                    rhs.push(')');
                }
            }
            '|' => {
                if in_group == 0 {
                    right = true;
                }
            }
            a => {
                if !right {
                    lhs.push(a);
                }else{
                    rhs.push(a);
                }
            }
        }
        temp = regex.next();
    }
    let rhs_str: String = rhs.iter().collect();
    Regex{
        content: Alternator{
            lhs: Expression::new(lhs.iter().collect()),
            rhs: match rhs_str.as_str() {
                "" => None,
                a => Some(Expression::new(a.to_string())),
            }
        }
    }
}
