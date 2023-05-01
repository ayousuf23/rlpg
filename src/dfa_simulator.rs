use crate::{dfa_builder::DFANode, nfa::TransitionKind, grammar2::Symbol};

pub struct DFASimulator;

impl DFASimulator {

    pub unsafe fn simulate_dfa_and_get_tokens(node: *mut DFANode, string: &str) -> (bool, Vec<Symbol>)
    {
        let seq: Vec<char> = string.chars().collect();
        let mut index = 0;
        // Lock the node
        let mut next = node;

        let mut tokens = Vec::new();

        while seq.len() > 0 && index <= seq.len() - 1
        {
            //println!("DFA Index: {}", index);
            // Get transition for any char or next char
            if let Some(dest) = (*next).raw_transitions.get(&TransitionKind::Character(seq[index]))
            {
                next = *dest;
            }
            else if let Some(dest) = (*next).raw_transitions.get(&TransitionKind::AnyChar) {
                next = *dest;
            }
            else {
                // If we reached the end of the DFA and arrived at an acceptance state
                println!("here");
                if let crate::dfa_builder::DFANodeKind::Accept(token) = &(*next).kind 
                {
                    if !token.is_empty() {
                        tokens.push(Symbol{name: token.to_string(), is_terminal: true});
                    }
                    next = node;
                    // Add CONTINUE here
                    continue;
                } else {
                    return (false, tokens);
                }
            }

            index += 1;
        }

        //println!("{:?}", (*next).states);
        //println!("{:?}", (*next).kind);

        // Get last node
        if let crate::dfa_builder::DFANodeKind::Accept(token) = &(*next).kind 
        {
            if !token.is_empty() {
                tokens.push(Symbol {name: token.to_string(), is_terminal: true});
            }
            return (true, tokens);
        } else {
            return (false, tokens);
        }
        //return (true, tokens);
    }
    
    pub unsafe fn simulate_dfa(node: *mut DFANode, string: &str) -> bool
    {
        let seq: Vec<char> = string.chars().collect();
        let mut index = 0;
        // Lock the node
        let mut next = node;

        while seq.len() > 0 && index <= seq.len() - 1
        {
            // Get transition for any char or next char
            if let Some(dest) = (*next).raw_transitions.get(&TransitionKind::Character(seq[index]))
            {
                next = *dest;
            }
            else if let Some(dest) = (*next).raw_transitions.get(&TransitionKind::AnyChar) {
                next = *dest;
            }
            else {
                return false;
            }

            index += 1;
        }

        // Get last node
        if let crate::dfa_builder::DFANodeKind::Accept(_) = &(*next).kind 
        {
            return true;
        } else {
            return false;
        }
    }
}