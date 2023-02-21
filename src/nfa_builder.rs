use std::{rc::Rc, sync::Mutex};

use crate::{nfa::{NFA, NFANode, Transition, TransitionKind, NFANodeKind}, node::Node, node_kind::NodeKind};

pub struct NFABuilder;

impl NFABuilder {

    pub fn build(node: &Node) -> Option<NFA> {
        return match &node.kind {
            NodeKind::Base => NFABuilder::build_from_base(node),
            NodeKind::RegEx => NFABuilder::build_from_regex(node),
            NodeKind::Root => NFABuilder::build_from_regex(node),
            NodeKind::BaseAnyChar => NFABuilder::build_from_base(node),
            NodeKind::Middle => NFABuilder::build_from_middle(node),
            NodeKind::MiddlePlus => NFABuilder::build_from_middle_plus(node),
            NodeKind::Star => NFABuilder::build_from_start(node),
            _ => panic!()
        };
    }

    pub fn build_from_regex(node: &Node) -> Option<NFA> {
        // What we want to do is create a transition from the end of one node to the start of another
        let mut first_start = None;
        let mut last_end: Option<Rc<Mutex<NFANode>>> = None;

        for child in &node.children {
            // Create an NFA for the child
            let child_nfa = NFABuilder::build(child.as_ref());
            if let None = child_nfa {
                continue;
            }
            let child_nfa = child_nfa.unwrap();

            if let None = first_start {
                first_start = Some(Rc::clone(&child_nfa.start));
            }

            if let Some(prev) = last_end {
                // Attatch the prev->end to current->start via empty transition
                let trans = Transition {
                    kind: TransitionKind::Empty,
                    destination: Rc::clone(&child_nfa.start),
                };
                let mut prev = prev.as_ref().lock().unwrap();
                prev.transitions.push(trans);
                prev.kind = NFANodeKind::Intersection;
            }

            last_end = Some(child_nfa.end);
        }

        if let Some(start) = first_start {
            if let Some(end) = last_end {
                return Some(NFA {
                    start,
                    end
                });
            }
        }

        None
    }

    pub fn build_from_middle(node: &Node) -> Option<NFA> {
        return NFABuilder::build(&node.children[0]);
    }

    pub fn build_from_middle_plus(node: &Node) -> Option<NFA> {
        // Build its child first
        let child_node = &node.children[0];
        if let Some(child) = NFABuilder::build(&child_node) {
            // Add a transition from end to start
            let trans = Transition {
                kind: TransitionKind::Empty,
                destination: Rc::clone(&child.start),
            };
            let mut end = child.end.as_ref().lock().unwrap();
            end.transitions.push(trans);
            drop(end);
            return Some(child)
        }
        None
    }

    pub fn build_from_start(node: &Node) -> Option<NFA> {
        // Build like a plus node
        if let Some(nfa) = NFABuilder::build_from_middle_plus(node) {
            // Add a transition from start to end
            let trans = Transition {
                kind: TransitionKind::StrictEmpty,
                destination: Rc::clone(&nfa.end),
            };
            let mut start = nfa.start.as_ref().lock().unwrap();
            start.transitions.push(trans);
            drop(start);
            return Some(nfa)
        }
        None
    }   

    pub fn build_from_base<'a>(node: &'a Node) -> Option<NFA> {
        // Create a start node
        let start = Rc::new(Mutex::new(NFANode::new_start()));

        // Create an end node
        let end = Rc::new(Mutex::new(NFANode::new_end()));

        let mut nfa = NFA {
            start,
            end,
        };

        let trans_kind = match &node.kind {
            NodeKind::Base => TransitionKind::Character(node.data.to_string().chars().nth(0).unwrap()),
            NodeKind::BaseAnyChar => TransitionKind::AnyChar,
            _ => panic!("Unexpected!"),
        };

        // Create transition from start to end via letter
        let transition = Transition {
            destination: Rc::clone(&nfa.end),
            kind: trans_kind,
        };

        nfa.start.as_ref().lock().unwrap().transitions.push(transition);
        
        // Return an NFA
        return Some(nfa);
    }
}