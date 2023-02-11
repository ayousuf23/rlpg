use std::{rc::Rc, sync::Mutex};


pub struct Transition {
    pub character: char,
    pub destination: Rc<Mutex<NFANode>>,
}

pub enum NFANodeKind {
    Start,
    End,
    Regular
}

pub struct NFANode {
    kind: NFANodeKind,
    data: String,
    pub transitions: Vec<Transition>,
}

pub struct NFA {
    pub start: Rc<Mutex<NFANode>>,
    pub end: Rc<Mutex<NFANode>>,
}

impl NFANode {
    pub fn new_regular<'a>(data: String) -> NFANode {
        NFANode {
            kind: NFANodeKind::Regular,
            data,
            transitions: Vec::new(),
        }
    }

    pub fn new_start<'a>() -> NFANode {
        NFANode {
            kind: NFANodeKind::Start,
            data: "Start".to_string(),
            transitions: Vec::new(),
        }
    }

    pub fn new_end<'a>() -> NFANode {
        NFANode {
            kind: NFANodeKind::End,
            data: "End".to_string(),
            transitions: Vec::new(),
        }
    }
}