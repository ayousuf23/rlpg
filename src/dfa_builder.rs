use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::nfa;
use crate::nfa::NFANodeKind;
use crate::nfa::Transition;
use crate::nfa::{NFA, NFANode, TransitionKind};

pub struct DFABuilder {
    pub nodes: HashMap<String, Rc<Mutex<DFANode>>>, // Map Node ID to backing store
    pub raw_nodes: HashMap<String, *mut DFANode>,
}

#[derive(Debug)]
pub enum DFANodeKind {
    Nonacccept,
    Accept(String),
}

pub struct DFANode {
    pub states: BTreeSet<i32>,
    pub nodes: Vec<Rc<Mutex<NFANode>>>,
    pub transitions: HashMap<TransitionKind, Rc<Mutex<DFANode>>>,
    pub raw_transitions: HashMap<TransitionKind, *mut DFANode>,
    pub kind: DFANodeKind,
}

impl DFANode {
    pub fn new(states: BTreeSet<i32>, nodes: Vec<Rc<Mutex<NFANode>>>, kind: DFANodeKind) -> DFANode
    {
        DFANode { states: states, nodes, transitions: HashMap::new(), raw_transitions: HashMap::new(), kind}
    }

    pub fn print(node: Rc<Mutex<DFANode>>)
    {
        let locked = node.lock().unwrap();
        println!("{:?}", locked.states);
    }
}

impl DFABuilder {

    pub unsafe fn convert_nfa_to_dfa(nfa: NFA) -> *mut DFANode
    {
        let mut dfa_builder = DFABuilder {
            nodes: HashMap::new(),
            raw_nodes: HashMap::new(),
        };
        return dfa_builder.convert_nfa_to_dfa_raw(nfa);
    }

    pub unsafe fn convert_nfa_to_dfa_raw(&mut self, nfa: NFA) -> *mut DFANode
    {
        // Create q0
        let mut q0 = self.single_nfa_to_dfa_node_raw(nfa.start);
        let mut q0_trans = self.dfa_raw_get_trans(q0);
        // Get epsilon closure of q0
        if let Some(q0_empty_nodes) = q0_trans.get_mut(&TransitionKind::Empty)
        {
            for node in &(*q0).nodes
            {
                q0_empty_nodes.push((Rc::clone(node), i32::MAX - 1));
            }
            q0 = self.get_epsilon_raw(q0_empty_nodes);
        }

        let mut work_list: Vec<*mut DFANode> = Vec::new();
        work_list.push(q0);
        let mut seen: BTreeSet<*const DFANode> = BTreeSet::new();
        seen.insert(q0);

        while let Some(node) = work_list.pop()
        {
            let transitions = self.dfa_raw_get_trans(node);

            for (trans, dests) in transitions
            {
                if let TransitionKind::Character(c) = &trans
                {
                    let d = c;
                }
                if trans != TransitionKind::Empty {
                    // Get the epsilon closure
                    let epsilon = self.get_epsilon_raw(&dests);
                    (*node).raw_transitions.insert(trans, epsilon);

                    if seen.insert(epsilon)
                    {
                        work_list.push(epsilon);
                    }
                }
            }

        }

        return q0;
    }

    unsafe fn get_epsilon_raw(&mut self, node: &Vec<(Rc<Mutex<NFANode>>, i32)>) -> *mut DFANode
    {
        let mut seen: BTreeSet<i32> = BTreeSet::new();
        let mut nodes: Vec<Rc<Mutex<NFANode>>> = Vec::new();
        let mut stack: Vec<(Rc<Mutex<NFANode>>, i32)> = Vec::new();
        let mut min_priority = i32::MAX;
        let mut kind = DFANodeKind::Nonacccept;

        for (nfa_node, priority) in node {
            stack.push((Rc::clone(nfa_node), *priority));
        }

        while let Some((front, priority)) = stack.pop()
        {
            let locked = front.lock().unwrap();
            let id = locked.id;
            if !seen.insert(locked.id)
            {
                // The node was already seen
                continue;
            }

            // Check whether this is an accepting state or not
            if priority < min_priority {
                match &locked.kind {
                    NFANodeKind::EndWithToken(token) => {
                        min_priority = priority;
                        kind = DFANodeKind::Accept(token.to_string());
                    },
                    NFANodeKind::End => {
                        min_priority = priority;
                        kind = DFANodeKind::Accept("".to_string());
                    },
                    _ => (),
                }
            }

            // Add node to nodes
            drop(locked);
            nodes.push(Rc::clone(&front));
            let locked = front.lock().unwrap();
            
            for trans in &locked.transitions
            {
                if let TransitionKind::Empty = trans.kind {
                    stack.push((trans.destination.clone(), trans.priority));
                }
            }

        }
        return self.to_dfa_node_raw(seen, nodes, kind);
    }

    // Get the transitions of this node based on underlying NFA transitions
    unsafe fn dfa_raw_get_trans(&mut self, node: *const DFANode) -> HashMap<TransitionKind, Vec<(Rc<Mutex<NFANode>>, i32)>>
    {
        //println!("DFA node get trans {:?}", (*node).states);
        let mut transitions: HashMap<TransitionKind, Vec<(Rc<Mutex<NFANode>>, i32)>> = HashMap::new();
        for nfa_node in &(*node).nodes
        {
            // Add a transition for each transition of each nfa_node
            let locked = nfa_node.lock().unwrap();
            let id = locked.id;
            // store temp
            let mut temp = Vec::new();
            for trans in &locked.transitions
            {
                temp.push((Rc::clone(&trans.destination), trans.priority, trans.kind.clone()));
                /*if let Some(dests) = transitions.get_mut(&trans.kind)
                {
                    dests.push((Rc::clone(&trans.destination), trans.priority));
                }
                else {
                    let mut dests = Vec::new();
                    dests.push((Rc::clone(&trans.destination), trans.priority));
                    transitions.insert(trans.kind.clone(), dests);
                }*/
            }

            drop(locked);
            for (dest, priority, kind) in temp {

                let locked_dest = dest.lock().unwrap();
                //println!("Trans kind: {:?}, Node ID: {}", kind, locked_dest.id);
                if let Some(dests) = transitions.get_mut(&kind)
                {
                    dests.push((Rc::clone(&dest), priority));
                }
                else {
                    let mut dests = Vec::new();
                    dests.push((Rc::clone(&dest), priority));
                    transitions.insert(kind.clone(), dests);
                }
            }
        }
        return transitions;
    }

    fn to_dfa_node_raw(&mut self, ids: BTreeSet<i32>, nodes: Vec<Rc<Mutex<NFANode>>>, kind: DFANodeKind) -> *mut DFANode
    {
        // First compute the id
        let id = DFABuilder::compute_dfa_node_id(&ids);
        if let Some(dfa_node) = self.raw_nodes.get(&id)
        {
            return dfa_node.clone();
        }
        let dfa_node = Box::into_raw(Box::new(DFANode::new(ids, nodes, kind)));
        self.raw_nodes.insert(id, dfa_node);
        return dfa_node;
    }

    fn single_nfa_to_dfa_node_raw(&mut self, node: Rc<Mutex<NFANode>>) -> *mut DFANode
    {
        let locked_node = node.lock().unwrap();
        let kind = match &locked_node.kind {
            NFANodeKind::EndWithToken(token) => DFANodeKind::Accept(token.to_string()),
            NFANodeKind::End => DFANodeKind::Accept("".to_string()),
            _ => DFANodeKind::Nonacccept,
        };
        let mut states: BTreeSet<i32> = BTreeSet::new();
        states.insert(locked_node.id);
        drop(locked_node);
        let mut nodes = Vec::new();
        nodes.push(node);
        return self.to_dfa_node_raw(states, nodes, kind);
    }

    fn compute_dfa_node_id(node_ids: &BTreeSet<i32>) -> String
    {
        let mut id = String::new();
        for n_id in node_ids
        {
            let format = format!("{},", n_id);
            id.push_str(&format);
        }
        return id;
    }
}