use std::{sync::Mutex, rc::Rc};

use crate::{dfa_builder::DFANode, nfa::TransitionKind};

pub struct DFASimulator;

impl DFASimulator {
    
    pub fn simulate_dfa(node: Rc<Mutex<DFANode>>, seq: Vec<char>)
    {
        let mut index = 0;
        // Lock the node
        let mut next = node;

        while index <= seq.len() - 2
        {
            let mut locked = next.lock().unwrap();

            // Get transition for any char or next char
            if let Some(dest) = locked.transitions.remove(&TransitionKind::Character(seq[index]))
            {
                drop(locked);
                next = dest;
            }
            else if let Some(dest) = locked.transitions.remove(&TransitionKind::AnyChar) {
                drop(locked);
                next = dest;
            }
            else {
                println!("Failed!");
            }

            index += 1;
        }

        // Print last node
        DFANode::print(next);
    }
}