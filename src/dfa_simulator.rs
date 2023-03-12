use std::{sync::Mutex, rc::Rc};

use crate::{dfa_builder::DFANode, nfa::TransitionKind};

pub struct DFASimulator;

impl DFASimulator {
    
    pub unsafe fn simulate_dfa(node: *mut DFANode, seq: Vec<char>)
    {
        let mut index = 0;
        // Lock the node
        let mut next = node;

        while index <= seq.len() - 1
        {
            println!("{:?}", (*next).states);
            println!("{:?}", (*next).raw_transitions);
            // Get transition for any char or next char
            if let Some(dest) = (*next).raw_transitions.remove(&TransitionKind::Character(seq[index]))
            {
                next = dest;
            }
            else if let Some(dest) = (*next).raw_transitions.remove(&TransitionKind::AnyChar) {
                next = dest;
            }
            else {
                println!("Failed!");
            }

            index += 1;
        }

        // Print last node
        println!("{:?}", (*next).kind);
    }
}