use std::{rc::Rc, sync::Mutex, collections::VecDeque};

use crate::{file_parser::Rule, regex_parser::RegExParser, NFABuilder, token::Token};

#[derive(Eq,PartialEq, Debug, Hash, Clone)]
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
    pub priority: i32,
    pub id: i32,
}

#[derive(Eq,PartialEq, Debug)]
pub enum NFANodeKind {
    Start,
    End,
    EndWithToken(String),
    Regular,
    Intersection,
}

#[derive(Debug)]
pub struct NFANode {
    pub kind: NFANodeKind,
    pub data: String,
    pub transitions: Vec<Transition>,
    pub id: i32,
}

#[derive(Debug)]
pub struct NFASimState {
    pub destination: Rc<Mutex<NFANode>>,
    pub start_i: usize,
    pub end_i: usize,
}

#[derive(Debug)]
pub struct NFA {
    pub start: Rc<Mutex<NFANode>>,
    pub end: Rc<Mutex<NFANode>>,
}

impl Transition {
    pub fn new(destination: Rc<Mutex<NFANode>>, kind: TransitionKind, priority: i32) -> Transition
    {
        static counter: i32 = 0;
        counter += 1;
        return Transition {destination, kind, priority, id: counter - 1};
    }

    pub fn copy(&self) -> Transition
    {
        return Transition {destination: Rc::clone(&self.destination), kind: self.kind,
            priority: self.priority, id: self.id};
    }
}

impl NFANode {
    pub fn new(kind: NFANodeKind, data: String) -> NFANode {
        static counter: i32 = 0;
        counter += 1;
        NFANode { kind: kind, data: data, transitions: Vec::new(), id: counter - 1 }
    }

    pub fn new_regular(data: String) -> NFANode {
        NFANode::new(NFANodeKind::Regular, data)
    }

    pub fn new_start() -> NFANode {
        NFANode::new(NFANodeKind::Start, "Start".to_string())
    }

    pub fn new_end() -> NFANode {
        NFANode::new(NFANodeKind::End, "End".to_string())
    }

    pub fn add_transition_to(&mut self, destination: Rc<Mutex<NFANode>>, transition_kind: TransitionKind, priority: i32)
    {
        self.transitions.push(Transition::new(destination, transition_kind, priority));
    }

    pub fn simulate(node: Rc<Mutex<NFANode>>, chars: &Vec<char>, index: usize) -> bool {

        let mut stack: VecDeque<(Rc<Mutex<NFANode>>, usize)> = VecDeque::new();
        stack.push_back((node, index));

        while let Some((curr, index)) = stack.pop_front() {
            let mut curr_node = curr.as_ref().lock().unwrap();

            let mut char = None;

            //println!("{:?}", curr_node.kind);
            //println!("{}", index);
            if index >= chars.len()
            {
                if curr_node.kind == NFANodeKind::End {
                    return true;
                }
            } else {
                char = Some(chars[index]);
            }

            
            // See if there is a transition on char
            for trans in &mut curr_node.transitions {

                let new_index = match trans.kind {
                    TransitionKind::AnyChar if char.is_some() => index + 1,
                    TransitionKind::Character(trans_char) if char.is_some() && trans_char == char.unwrap() => index + 1,
                    TransitionKind::Empty => index,
                    _ => continue,
                };
                
                stack.push_back((Rc::clone(&trans.destination), new_index));
            }
        }
        return false;
    }

    pub fn simulate_and_get_all_tokens(node: Rc<Mutex<NFANode>>, chars: &Vec<char>, index: usize) -> (bool, Vec<Token>) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut success = false;
        let mut stack: VecDeque<NFASimState> = VecDeque::new();
        stack.push_back(NFASimState { destination: node, start_i: 0, end_i: 0 });
        let mut min_start_i = 0;

        while let Some(mut state) = stack.pop_back() {
            if state.start_i < min_start_i {
                continue;
            }

            let mut curr_node = state.destination.as_ref().lock().unwrap();

            let mut char = None;

            if let NFANodeKind::EndWithToken(token) = &curr_node.kind  {
               
                let lexeme = chars[state.start_i..state.end_i].into_iter().collect();
                tokens.push(Token {name: token.to_string(), lexeme, line: 1, start_col: state.start_i, end_col: state.end_i - 1});
            }
            else if let NFANodeKind::Start = &curr_node.kind {
                state.start_i = state.end_i;
                min_start_i = state.end_i;
            }

            if state.end_i >= chars.len()
            {
                match &curr_node.kind {
                    NFANodeKind::EndWithToken(token) => {
                        success = true;
                        break;
                    },
                    NFANodeKind::End => {
                        success = true;
                        break;
                    },
                    _ => (),
                }
            } else {
                char = Some(chars[state.end_i]);
            }
            
            // See if there is a transition on char
            curr_node.transitions.sort_by_key(|x| -x.priority);
            for trans in &mut curr_node.transitions {
                let new_index = match trans.kind {
                    TransitionKind::AnyChar if char.is_some() && !char.unwrap().is_whitespace() => state.end_i + 1,
                    TransitionKind::Character(trans_char) if char.is_some() && trans_char == char.unwrap() => state.end_i + 1,
                    TransitionKind::Empty => state.end_i,
                    _ => continue,
                };
                stack.push_back(NFASimState {destination: Rc::clone(&trans.destination), start_i: state.start_i, end_i: new_index});
            }
        }
        return (success, tokens);
    }

}

impl NFA {
    pub fn simulate(&self, string: &str) -> bool {
        let chars: Vec<char> = string.chars().collect();
        /*let start = self.start.as_ref().lock().unwrap();
        return start.simulate(&chars, 0);*/
        return NFANode::simulate(Rc::clone(&self.start), &chars, 0);
    }

    pub fn simulateAndGetToken(&self, string: &str) -> (bool, Vec<Token>) {
        let chars: Vec<char> = string.chars().collect();
        return NFANode::simulate_and_get_all_tokens(Rc::clone(&self.start), &chars, 0);
    }

    pub fn build_from_rules(rules: &Vec<Rule>) -> Option<NFA> {
        if rules.len() == 0 {
            return None;
        }

        // Create start node
        let mut start = Rc::new(Mutex::new(NFANode::new_start()));
        
        for rule in rules {
            // Create parse tree
            let mut parser = RegExParser::new(&rule.regex);
            let mut parse_root = parser.parse();

            // Create NFA
            let mut nfa: NFA = NFABuilder::build(&parse_root).expect("Error");

            // Combine with start node
            let mut nfa_start = nfa.start.as_ref().lock().unwrap();
            nfa_start.kind = NFANodeKind::Intersection;
            drop(nfa_start);

            let mut nfa_end = nfa.end.as_ref().lock().unwrap();

            // change end node to EndsWithToken
            if let crate::file_parser::RuleKind::Named(name) = &rule.kind {
                nfa_end.kind = NFANodeKind::EndWithToken(name.to_string());
            }
            nfa_end.add_transition_to(Rc::clone(&start), TransitionKind::Empty, rule.priority);


            // Add transition from start to nfa_start
            let mut start_unlocked = start.as_ref().lock().unwrap();
            start_unlocked.add_transition_to(nfa.start, TransitionKind::Empty, rule.priority);
        }
        return Some(NFA {
            start,
            end: Rc::new(Mutex::new(NFANode::new_end())),
        });
    }
}