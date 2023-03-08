use std::{rc::Rc, sync::Mutex};

use crate::{nfa::{NFA, NFANode, Transition, TransitionKind, NFANodeKind}, node::Node, node_kind::NodeKind};

pub struct NFABuilder;

impl NFABuilder {

    pub unsafe fn build(node: &Node) -> Option<NFA> {
        return match &node.kind {
            NodeKind::Base => NFABuilder::build_from_base(node),
            NodeKind::RegEx => NFABuilder::build_from_regex(node),
            NodeKind::Root => NFABuilder::build_from_regex(node),
            NodeKind::BaseAnyChar => NFABuilder::build_from_base(node),
            NodeKind::Middle => NFABuilder::build_from_middle(node),
            NodeKind::MiddlePlus => NFABuilder::build_from_middle_plus(node),
            NodeKind::Star => NFABuilder::build_from_star(node),
            NodeKind::QuestionMark => NFABuilder::build_from_question_mark(node),
            NodeKind::Parentheses => NFABuilder::build_from_parentheses(node),
            NodeKind::Bracket => NFABuilder::build_or_of_child_nodes(node),
            NodeKind::High => NFABuilder::build_or_of_child_nodes(node),
            _ => panic!()
        };
    }

    pub unsafe fn build_from_regex(node: &Node) -> Option<NFA> {
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
            } else {
                NFABuilder::node_change_kind_and_add_transition(&child_nfa.start, Some(NFANodeKind::Intersection), None);
            }

            if let Some(prev) = last_end {
                // Attatch the prev->end to current->start via empty transition
                let trans = Transition::new(Rc::clone(&child_nfa.start), TransitionKind::Empty, 1);
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

    unsafe fn build_from_high(node: &Node) -> Option<NFA> {
        return NFABuilder::build_or_of_child_nodes(node);
    }

    pub unsafe fn build_from_middle(node: &Node) -> Option<NFA> {
        return NFABuilder::build(&node.children[0]);
    }

    pub unsafe fn build_from_middle_plus(node: &Node) -> Option<NFA> {
        // Build its child first
        let child_node = &node.children[0];
        if let Some(child) = NFABuilder::build(&child_node) {
            // Add a transition from end to start
            let trans = Transition::new(Rc::clone(&child.start), TransitionKind::Empty, 1);
            let mut end = child.end.as_ref().lock().unwrap();
            end.transitions.push(trans);
            drop(end);
            return Some(child)
        }
        None
    }

    pub unsafe fn build_from_star(node: &Node) -> Option<NFA> {
        // Build like a plus node
        if let Some(nfa) = NFABuilder::build_from_middle_plus(node) {
            // Add a new start node 
            let mut new_start = NFANode::new_start();

            NFABuilder::node_change_kind_and_add_transition(&nfa.start, Some(NFANodeKind::Intersection), None);

            // Add empty transition from new_start to end
            new_start.add_transition_to(Rc::clone(&nfa.end), TransitionKind::Empty, 1);

            // Add empty transition from new_start to start
            new_start.add_transition_to(nfa.start, TransitionKind::Empty, 1);

            return Some(NFA {start: Rc::new(Mutex::new(new_start)), end: nfa.end});
        }
        None
    }

    pub unsafe fn build_from_question_mark(node: &Node) -> Option<NFA> {
        if let Some(nfa) = NFABuilder::build(node.children[0].as_ref()) {
            let mut start = nfa.start.lock().unwrap();

            // Add empty transition from new_start to end
            start.add_transition_to(Rc::clone(&nfa.end), TransitionKind::Empty, 1);
            drop(start);
            return Some(nfa);
        }
        None
    }

    pub unsafe fn build_from_parentheses(node: &Node) -> Option<NFA> {
        return NFABuilder::build_from_regex(node);
    }

    pub unsafe fn build_or_of_child_nodes(node: &Node) -> Option<NFA> {
        // Create a new start node
        let mut real_start = Rc::new(Mutex::new(NFANode::new_start()));
        let mut start = real_start.lock().unwrap();

        // Create a new end node
        let mut end = Rc::new(Mutex::new(NFANode::new_end()));

        for child_node in &node.children {
            // Build the child
            if let Some(built_child) = NFABuilder::build(&child_node) {
                // Change start node to intersection
                NFABuilder::node_change_kind_and_add_transition(&built_child.start, Some(NFANodeKind::Intersection), None);

                // Change the end to an intersection
                let mut built_child_end = built_child.end.lock().unwrap();
                built_child_end.kind = NFANodeKind::Intersection;
                // Add empty transition from child end to end
                built_child_end.add_transition_to(Rc::clone(&end), TransitionKind::Empty, 1);

                // Add empty transition to child start
                start.add_transition_to(built_child.start, TransitionKind::Empty, 1);
            }
        }
        drop(start);

        return Some(NFA {start: real_start, end});
    }

    pub unsafe fn build_from_base<'a>(node: &'a Node) -> Option<NFA> {
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
        let transition = Transition::new(Rc::clone(&nfa.end), trans_kind, 1);

        nfa.start.as_ref().lock().unwrap().transitions.push(transition);
        
        // Return an NFA
        return Some(nfa);
    }

    fn node_change_kind_and_add_transition(node: &Rc<Mutex<NFANode>>, new_kind: Option<NFANodeKind>, trans_to_add: Option<Transition>)
    {
        let mut n = node.as_ref().lock().unwrap();
        if let Some(kind) = new_kind {
            n.kind = kind;
        }
        if let Some(trans) = trans_to_add {
            n.transitions.push(trans);
        }
    }
}