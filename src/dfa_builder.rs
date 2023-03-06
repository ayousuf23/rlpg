use std::rc::Rc;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::nfa::{NFA, NFANode, TransitionKind};

struct DFABuilder;

struct DFANode {
    pub states: HashSet<i32>,
}

impl DFANode {
    pub fn new(states: HashSet<i32>) -> DFANode
    {
        DFANode { states: states }
    }
}

impl DFABuilder {

    fn convert_nfa_to_dfa(nfa: NFA) {
        let mut seen: Vec<DFANode> = Vec::new();
        let mut work_list: VecDeque<DFANode> = VecDeque::new();

        while !work_list.is_empty()
        {
            // Get q
            let q = work_list.pop_front();
        }
    }

    fn get_epsilon_closure(node: Rc<Mutex<NFANode>>) -> DFANode {
        // Get all nodes reaching from empty transitions
        let reachable: HashSet<i32> = HashSet::new();
        let mut stack: VecDeque<Rc<Mutex<NFANode>>> = VecDeque::new();
        stack.push_back(node);
        let mut set: HashSet<i32> = HashSet::new();

        while let Some(reachable_node) = stack.pop_front()
        {
            // Check its transitions for empty transitions
            let locked = reachable_node.lock().unwrap();
            reachable.insert(locked.id);
            for trans in &locked.transitions {
                if !set.insert(trans.id) {
                    continue;
                }
                if let TransitionKind::Empty = trans.kind {
                    // Add to stack
                    stack.push_back(trans.destination);
                }
            }
        }
        return DFANode::new(reachable);
    }
}