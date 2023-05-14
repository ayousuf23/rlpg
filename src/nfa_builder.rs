use std::{rc::Rc, sync::Mutex, error::Error};

use crate::{nfa::{NFA, NFANode, Transition, TransitionKind, NFANodeKind}, node::Node, node_kind::NodeKind, grammar2::Empty};

#[derive(Debug)]
pub enum NFABuilderError 
{
    NoRules,
    RegExError,
    DuplicateNamedRule,
    NoChildren,
    UnexpectedNodeKind,
}

impl core::fmt::Display for NFABuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NFABuilderError
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        return match self {
            Self::NoRules => "There is no rules for which to generate an NFA.",
            Self::RegExError => "There was an error parsing the regex patter.",
            Self::DuplicateNamedRule => "Names for rules must be unique. There are at least two rules with the same name.",
            Self::NoChildren => "The node has 0 children and while at least one child was expected.",
            Self::UnexpectedNodeKind => "A parser node of an unexpected type was encountered.",
        };
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub struct NFABuilder;

impl NFABuilder {

    pub unsafe fn build(node: &Node) -> Result<NFA, NFABuilderError> {
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

    pub unsafe fn build_from_regex(node: &Node) -> Result<NFA, NFABuilderError> {        
        // What we want to do is create a transition from the end of one node to the start of another
        let mut first_start: Option<Rc<Mutex<NFANode>>> = None;
        let mut last_end: Option<Rc<Mutex<NFANode>>> = None;

        let mut emptiness = Empty::PossiblyEmpty;

        for (index, child) in node.children.iter().enumerate() {
            // Create an NFA for the child
            let child_nfa = NFABuilder::build(child.as_ref());
            if child_nfa.is_err() {
                return child_nfa;
            }
            let child_nfa = child_nfa.unwrap();

            if index == 0 {
                first_start = Some(Rc::clone(&child_nfa.start));
            } else {
                NFABuilder::node_change_kind_and_add_transition(&child_nfa.start, Some(NFANodeKind::Intersection), None);
            }

            // Check emptiness
            let child_nfa_start = child_nfa.start.lock().unwrap();
            if let Empty::NonEmpty = child_nfa_start.emptiness {
                emptiness = Empty::NonEmpty;
            }

            if index > 0 {
                // Attatch the prev->end to current->start via empty transition
                let trans = Transition::new(Rc::clone(&child_nfa.start), TransitionKind::Empty, 1);
                let last_end_unwrap = last_end.unwrap();
                let mut prev = last_end_unwrap.lock().unwrap();
                prev.transitions.push(trans);
                prev.kind = NFANodeKind::Intersection;
            }

            last_end = Some(child_nfa.end);
        }

        if let Some(first_start) = first_start {
            let mut first_start_locked = first_start.lock().unwrap();
            first_start_locked.emptiness = emptiness;
            drop(first_start_locked);
            return Ok(NFA {
                start: first_start,
                end: last_end.unwrap(),
            });
        }

        return Err(NFABuilderError::NoChildren);
    }

    unsafe fn build_from_high(node: &Node) -> Result<NFA, NFABuilderError> {
        return NFABuilder::build_or_of_child_nodes(node);
    }

    pub unsafe fn build_from_middle(node: &Node) -> Result<NFA, NFABuilderError> {
        return NFABuilder::build(&node.children[0]);
    }

    pub unsafe fn build_from_middle_plus(node: &Node) -> Result<NFA, NFABuilderError> {
        // Build its child first
        let child_node = &node.children[0];
        let built_child = match NFABuilder::build(&child_node) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        let trans = Transition::new(Rc::clone(&built_child.start), TransitionKind::Empty, 1);
        let mut end = built_child.end.as_ref().lock().unwrap();
        end.transitions.push(trans);
        drop(end);
        return Ok(built_child);
    }

    pub unsafe fn build_from_star(node: &Node) -> Result<NFA, NFABuilderError> {
        let nfa = match NFABuilder::build_from_middle_plus(node) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        let mut new_start = NFANode::new_start();
        NFABuilder::node_change_kind_and_add_transition(&nfa.start, Some(NFANodeKind::Intersection), None);

        // Add empty transition from new_start to end
        new_start.add_transition_to(Rc::clone(&nfa.end), TransitionKind::Empty, 1);
        new_start.emptiness = Empty::PossiblyEmpty;

        // Add empty transition from new_start to start
        new_start.add_transition_to(nfa.start, TransitionKind::Empty, 1);

        return Ok(NFA {start: Rc::new(Mutex::new(new_start)), end: nfa.end});
    }

    pub unsafe fn build_from_question_mark(node: &Node) -> Result<NFA, NFABuilderError> {
        let nfa = match NFABuilder::build(node.children[0].as_ref()) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };
        let mut start = nfa.start.lock().unwrap();
        start.emptiness = Empty::PossiblyEmpty;

        // Add empty transition from new_start to end
        start.add_transition_to(Rc::clone(&nfa.end), TransitionKind::Empty, 1);
        drop(start);
        return Ok(nfa);
    }

    pub unsafe fn build_from_parentheses(node: &Node) -> Result<NFA, NFABuilderError> {
        return NFABuilder::build_from_regex(node);
    }

    pub unsafe fn build_or_of_child_nodes(node: &Node) -> Result<NFA, NFABuilderError> {
        match node.children.len() {
            0 => return Err(NFABuilderError::NoChildren),
            1 =>  return NFABuilder::build(node.children[0].as_ref()),
            _ => (),
        }

        let mut emptiness = Empty::NonEmpty;
        
        // Create a new start node
        let real_start = Rc::new(Mutex::new(NFANode::new_start()));
        let mut start = real_start.lock().unwrap();
        //println!("OR Node ID: {}", start.id);

        // Create a new end node
        let end = Rc::new(Mutex::new(NFANode::new_end()));

        for child_node in &node.children {
            // Build the child
            let built_child = match NFABuilder::build(&child_node) {
                Ok(node) => node,
                Err(err) => return Err(err),
            };
            // Change start node to intersection
            NFABuilder::node_change_kind_and_add_transition(&built_child.start, Some(NFANodeKind::Intersection), None);

            // Check emptiness
            let lock_start = built_child.start.lock().unwrap();
            if let Empty::PossiblyEmpty = lock_start.emptiness {
                emptiness = Empty::PossiblyEmpty;
            }
            drop(lock_start);

            // Change the end to an intersection
            let mut built_child_end = built_child.end.lock().unwrap();
            built_child_end.kind = NFANodeKind::Intersection;
            // Add empty transition from child end to end
            built_child_end.add_transition_to(Rc::clone(&end), TransitionKind::Empty, 1);

            // Add empty transition to child start
            start.add_transition_to(built_child.start, TransitionKind::Empty, 1);
        }
        start.emptiness = emptiness;
        drop(start);
        
        return Ok(NFA {start: real_start, end});
    }

    pub unsafe fn build_from_base(node: &Node) -> Result<NFA, NFABuilderError> {
        // Create a start node
        //println!("Data: {}", node.data);
        let start = Rc::new(Mutex::new(NFANode::new_start()));

        // Create an end node
        let end = Rc::new(Mutex::new(NFANode::new_end()));

        let nfa = NFA {
            start,
            end,
        };

        let trans_kind = match &node.kind {
            NodeKind::Base => TransitionKind::Character(node.data.to_string().chars().nth(0).unwrap()),
            NodeKind::BaseAnyChar => TransitionKind::AnyChar,
            _ => return Err(NFABuilderError::UnexpectedNodeKind),
        };

        // Create transition from start to end via letter
        let transition = Transition::new(Rc::clone(&nfa.end), trans_kind, 1);

        let mut locked_start = nfa.start.as_ref().lock().unwrap();
        locked_start.transitions.push(transition);
        //println!("start: {}", locked_start.id);

        let locked_end = nfa.end.as_ref().lock().unwrap();
        //println!("end: {}", locked_end.id);
        drop(locked_end);
        drop(locked_start);
        // Return an NFA
        return Ok(nfa);
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