use std::{rc::Rc, sync::Mutex, collections::VecDeque};

#[derive(Eq,PartialEq, Debug)]
pub enum TransitionKind {
    Empty,
    StrictEmpty,
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
        let mut char = None;
        if index >= chars.len() {
            // Check if there an empty or strict empty transition first
            if self.kind == NFANodeKind::End {
                return true;
            }
        } else {
            char = Some(chars[index]);
        }

        
        // See if there is a transition on char
        for trans in &self.transitions {
            let new_index = match trans.kind {
                TransitionKind::AnyChar if char.is_some() => index + 1,
                TransitionKind::Character(trans_char) if char.is_some() && trans_char == char.unwrap() => index + 1,
                TransitionKind::Empty => index,
                TransitionKind::StrictEmpty if chars.len() == 0 => index,
                _ => panic!(),
            };
            
            let node = trans.destination.as_ref().lock().unwrap();
            if node.simulate(chars, new_index) {
                return true;
            }
        }

        if index >= chars.len() {
            return self.kind == NFANodeKind::End;
        }

        return false;
    }

    pub fn simulate2(node: Rc<Mutex<NFANode>>, chars: &Vec<char>, index: usize) -> bool {

        //let mut stack: Vec<(Rc<Mutex<NFANode>>, usize)> = Vec::new();
        

        let mut stack: VecDeque<(Rc<Mutex<NFANode>>, usize)> = VecDeque::new();
        stack.push_back((node, index));

        while let Some((curr, index)) = stack.pop_front() {
            let curr_node = curr.as_ref().lock().unwrap();

            let mut char = None;
            if index >= chars.len()
            {
                if curr_node.kind == NFANodeKind::End {
                    return true;
                }
            } else {
                char = Some(chars[index]);
            }

            
            // See if there is a transition on char
            for trans in &curr_node.transitions {
                let new_index = match trans.kind {
                    TransitionKind::AnyChar if char.is_some() => index + 1,
                    TransitionKind::Character(trans_char) if char.is_some() && trans_char == char.unwrap() => index + 1,
                    TransitionKind::Empty => index,
                    TransitionKind::StrictEmpty => index,
                    _ => continue,
                };
                
                stack.push_back((Rc::clone(&trans.destination), new_index));
            }
        }
        return false;
    }
}

impl NFA {
    pub fn simulate(&self, string: String) -> bool {
        let chars: Vec<char> = string.chars().collect();
        /*let start = self.start.as_ref().lock().unwrap();
        return start.simulate(&chars, 0);*/
        return NFANode::simulate2(Rc::clone(&self.start), &chars, 0);
    }
}