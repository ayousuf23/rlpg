use std::{rc::Rc, sync::Mutex};

#[derive(Eq,PartialEq, Debug)]
pub enum TransitionKind {
    Empty,
    Character(char),
    AnyChar,
}

#[derive(Debug)]
pub struct Transition {
    pub destination: Rc<Mutex<NFANode>>,
    pub kind: TransitionKind,
}

#[derive(Eq,PartialEq, Debug)]
pub enum NFANodeKind {
    Start,
    End,
    Regular,
    Intersection,
}

#[derive(Debug)]
pub struct NFANode {
    pub kind: NFANodeKind,
    data: String,
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
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

    pub fn simulate(&self, chars: &Vec<char>, index: usize) -> bool {
        if index >= chars.len() {
            return self.kind == NFANodeKind::End;
        }

        let char = chars[index];
        // See if there is a transition on char
        for trans in &self.transitions {
            let new_index = match trans.kind {
                TransitionKind::AnyChar => index + 1,
                TransitionKind::Character(trans_char) if trans_char == char => index + 1,
                TransitionKind::Empty => index,
                _ => continue,
            };
            
            let node = trans.destination.as_ref().lock().unwrap();
            if node.simulate(chars, new_index) {
                return true;
            }
        }

        return false;
    }
}

impl NFA {
    pub fn simulate(&self, string: String) -> bool {
        let chars: Vec<char> = string.chars().collect();
        let start = self.start.as_ref().lock().unwrap();
        return start.simulate(&chars, 0);
    }
}