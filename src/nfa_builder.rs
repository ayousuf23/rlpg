use std::{rc::Rc, sync::Mutex};

use crate::{nfa::{NFA, NFANode, Transition}, node::Node};

pub struct NFABuilder;

impl NFABuilder {

    fn build(&mut self) {

    }

    pub fn build_from_base<'a>(node: &'a Node) -> NFA {
        // Create a start node
        let start = Rc::new(Mutex::new(NFANode::new_start()));

        // Create an end node
        let end = Rc::new(Mutex::new(NFANode::new_end()));

        let mut nfa = NFA {
            start,
            end,
        };

        // Create transition from start to end via letter
        let transition = Transition {
            destination: Rc::clone(&nfa.end),
            character: node.data.to_string().chars().nth(0).unwrap(),
        };

        nfa.start.as_ref().lock().unwrap().transitions.push(transition);
        
        // Return an NFA
        return nfa;
    }
}