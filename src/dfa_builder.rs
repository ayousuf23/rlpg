use std::rc::Rc;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::nfa::{NFANode, TransitionKind};

struct DFABuilder;

struct DFANode {

}

impl DFABuilder {

    fn get_epsilon_closure(node: Rc<Mutex<NFANode>>) {
        // Get all nodes reaching from empty transitions
        let reachable: Vec<Rc<Mutex<NFANode>>> = Vec::new();
        let mut stack: VecDeque<Rc<Mutex<NFANode>>> = VecDeque::new();
        stack.push_back(node);
        let mut set: HashSet<i32> = HashSet::new();

        while let Some(reachable_node) = stack.pop_front()
        {
            // Check its transitions for empty transitions
            let locked = reachable_node.lock().unwrap();
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
    }
}