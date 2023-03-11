use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::nfa::Transition;
use crate::nfa::{NFA, NFANode, TransitionKind};

pub struct DFABuilder {
    pub nodes: HashMap<String, Rc<Mutex<DFANode>>>, // Map Node ID to backing store
    pub raw_nodes: HashMap<String, *mut DFANode>,
}

pub struct DFANode {
    pub states: HashSet<i32>,
    pub nodes: Vec<Rc<Mutex<NFANode>>>,
    pub transitions: HashMap<TransitionKind, Rc<Mutex<DFANode>>>,
    pub raw_transitions: HashMap<TransitionKind, *mut DFANode>,
}

impl DFANode {
    pub fn new(states: HashSet<i32>, nodes: Vec<Rc<Mutex<NFANode>>>) -> DFANode
    {
        DFANode { states: states, nodes, transitions: HashMap::new(), raw_transitions: HashMap::new()}
    }

    pub fn print(node: Rc<Mutex<DFANode>>)
    {
        let locked = node.lock().unwrap();
        println!("{:?}", locked.states);
    }
}

impl DFABuilder {

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
                q0_empty_nodes.push(Rc::clone(node));
            }
            q0 = self.get_epsilon_raw(q0_empty_nodes);
        }

        let mut work_list: Vec<*mut DFANode> = Vec::new();
        let mut seen: HashSet<*const DFANode> = HashSet::new();

        while let Some(node) = work_list.pop()
        {
            let transitions = self.dfa_raw_get_trans(node);

            for trans in transitions
            {
                if trans.0 != TransitionKind::Empty {
                    // Get the epsilon closure
                    let epsilon = self.get_epsilon_raw(&trans.1);
                    (*node).raw_transitions.insert(trans.0, epsilon);

                    if !seen.insert(epsilon)
                    {
                        work_list.push(epsilon);
                    }
                }
            }

        }

        return q0;
    }

    pub fn convert_nfa_to_dfa(&mut self, nfa: NFA) -> Rc<Mutex<DFANode>> {
        // Convert start node to a single DFA node
        let q0: Rc<Mutex<DFANode>> = self.single_nfa_to_dfa_node(nfa.start);
        let q0 = self.get_epsilon_closure(q0);

        let mut seen: HashSet<String> = HashSet::new();
        let mut work_list: VecDeque<Rc<Mutex<DFANode>>> = VecDeque::new();
        work_list.push_back(Rc::clone(&q0));

        while !work_list.is_empty()
        {
            // Remove q from work list
            let q = work_list.pop_front();
            if let None = q {continue;}
            let q = q.unwrap();

            // Fill the node's transitions
            let mut q_locked = q.lock().unwrap();
            if q_locked.transitions.is_empty() {
                drop(q_locked);
                self.dfa_node_fill_transitions(&q);
                q_locked = q.lock().unwrap();
            }

            // For each transition
            let mut transition: Vec<(TransitionKind, Rc<Mutex<DFANode>>)> = Vec::new(); 
            for (kind, dest) in &q_locked.transitions {
                if let TransitionKind::Empty = kind {
                    continue;
                }
                transition.push((kind.clone(), Rc::clone(dest)));
            }

            drop(q_locked);

            // For each transition
            for trans in transition {
                // Get the epsilon closure of each destination
                let dest_epsilon = self.get_epsilon_closure(trans.1);
                // Add transition from q to dest_epsilon
                let mut q_locked = q.lock().unwrap();
                q_locked.transitions.insert(trans.0, Rc::clone(&dest_epsilon));

                drop(q_locked);
                let dest_epsil_locked = dest_epsilon.lock().unwrap();
                // Compute the ID
                let id = DFABuilder::compute_dfa_node_id(&dest_epsil_locked.states);
                if seen.insert(id)
                {
                    drop(dest_epsil_locked);
                    work_list.push_back(dest_epsilon);
                }
            }
        }
        return q0;
    }

    fn get_epsilon_closure(&mut self, node: Rc<Mutex<DFANode>>) -> Rc<Mutex<DFANode>>
    {
        let mut seen: HashSet<String> = HashSet::new();
        let mut nodes: Vec<Rc<Mutex<DFANode>>> = Vec::new();
        let mut stack: Vec<Rc<Mutex<DFANode>>> = Vec::new();
        stack.push(node);

        while let Some(dfa_node) = stack.pop() {      
            // Lock node
            let locked = dfa_node.lock().unwrap();
            // Compute id
            let id = DFABuilder::compute_dfa_node_id(&locked.states);
            if !seen.insert(id)
            {
                continue;
            }

             // Fill transitions if not already filled
            if locked.transitions.is_empty() {
                drop(locked);
                self.dfa_node_fill_transitions(&dfa_node);
            } else {
                drop(locked);
            }

            // Lock node again
            let locked = dfa_node.lock().unwrap();

            // Follow the empty transitions
            if let Some(dest) = locked.transitions.get(&TransitionKind::Empty)
            {
                stack.push(Rc::clone(dest));
            }

            // Add it to nodes & seen
            drop(locked);
            nodes.push(dfa_node);
        }

        // Combine dfa nodes in nodes
        return self.combine_dfa_nodes(&nodes);
    }

    unsafe fn get_epsilon_raw(&mut self, node: &Vec<Rc<Mutex<NFANode>>>) -> *mut DFANode
    {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut nodes: Vec<Rc<Mutex<NFANode>>> = Vec::new();
        let mut stack: Vec<Rc<Mutex<NFANode>>> = Vec::new();
        for nfa_node in node {
            stack.push(Rc::clone(nfa_node));
        }

        while let Some(front) = stack.pop()
        {
            let locked = front.lock().unwrap();
            if !seen.insert(locked.id)
            {
                // The node was already seen
                continue;
            }

            // Add node to nodes
            drop(locked);
            nodes.push(Rc::clone(&front));
            let locked = front.lock().unwrap();
            
            for trans in &locked.transitions
            {
                if let TransitionKind::Empty = trans.kind {
                    stack.push(trans.destination.clone());
                }
            }

        }
        return self.to_dfa_node_raw(seen, nodes);
    }

    // Get the transitions of this node based on underlying NFA transitions
    unsafe fn dfa_raw_get_trans(&mut self, node: *const DFANode) -> HashMap<TransitionKind, Vec<Rc<Mutex<NFANode>>>>
    {
        let mut transitions: HashMap<TransitionKind, Vec<Rc<Mutex<NFANode>>>> = HashMap::new();
        for nfa_node in &(*node).nodes
        {
            // Add a transition for each transition of each nfa_node
            let locked = nfa_node.lock().unwrap();
            for trans in &locked.transitions
            {
                if let Some(dests) = transitions.get_mut(&trans.kind)
                {
                    dests.push(Rc::clone(&trans.destination));
                }
                else {
                    let mut dests = Vec::new();
                    dests.push(Rc::clone(&trans.destination));
                    transitions.insert(trans.kind.clone(), dests);
                }
            }
        }
        return transitions;
    }

    // Get all transitions from node 
    fn dfa_node_fill_transitions(&mut self, node: &Rc<Mutex<DFANode>>)
    {
        let mut seen_map: HashMap<TransitionKind, DFANode> = HashMap::new();
        let mut seen_nodes: HashMap<TransitionKind, String> = HashMap::new();
        // Store which node to check next
        let mut stack: Vec<Rc<Mutex<NFANode>>> = Vec::new();
        let mut locked = node.lock().unwrap();
        for backing_node in &locked.nodes {
            stack.push(Rc::clone(backing_node));
        }

        // Create a stack of transitions
        let mut trans_stack: Vec<Transition> = Vec::new();

        // Get first item from stack
        while let Some(top) = stack.pop()
        {
            // Lock it
            let locked_top = top.lock().unwrap();

            // Look through its transitions
            for trans in &locked_top.transitions
            {
                if trans.kind == TransitionKind::Empty {continue;}
                trans_stack.push(trans.copy());
            }
        }

        // Now look through the transition stack
        for trans in trans_stack
        {
            // Check if the destination was seen
            let dest = trans.destination.lock().unwrap();
            if let Some(trans_kind_seen) = seen_map.get_mut(&trans.kind)
            {
                if trans_kind_seen.states.insert(dest.id)
                {
                    drop(dest);
                    trans_kind_seen.nodes.push(trans.destination);
                }
            } else {
                let mut new_dfa = DFANode::new(HashSet::new(), Vec::new());
                let mut new_set: HashSet<i32> = HashSet::new();
                new_set.insert(dest.id);
                drop(dest);
                new_dfa.nodes.push(trans.destination);
                seen_map.insert(trans.kind, new_dfa);
            }
        }

        // Finally, create a node for each transition kind
        self.add_transitions_to_node(&mut locked, seen_map);
    }

    fn add_transitions_to_node(&mut self, node: &mut DFANode, trans: HashMap<TransitionKind, DFANode>)
    {
        for (key, value) in trans
        {
            // Check if there exists the same DFA node already
            let dfa = self.to_dfa_node(value.states, value.nodes);
            node.transitions.insert(key.clone(), dfa);
        }
    }

    fn to_dfa_node(&mut self, ids: HashSet<i32>, nodes: Vec<Rc<Mutex<NFANode>>>) -> Rc<Mutex<DFANode>>
    {
        // First compute the id
        let id = DFABuilder::compute_dfa_node_id(&ids);
        if let Some(dfa_node) = self.nodes.get(&id)
        {
            return Rc::clone(dfa_node);
        }
        let dfa_node = Rc::new(Mutex::new(DFANode::new(ids, nodes)));
        self.nodes.insert(id, Rc::clone(&dfa_node));
        return dfa_node;
    }

    fn to_dfa_node_raw(&mut self, ids: HashSet<i32>, nodes: Vec<Rc<Mutex<NFANode>>>) -> *mut DFANode
    {
        // First compute the id
        let id = DFABuilder::compute_dfa_node_id(&ids);
        if let Some(dfa_node) = self.raw_nodes.get(&id)
        {
            return dfa_node.clone();
        }
        let dfa_node = Box::into_raw(Box::new(DFANode::new(ids, nodes)));
        self.raw_nodes.insert(id, dfa_node);
        return dfa_node;
    }

    fn single_nfa_to_dfa_node(&mut self, node: Rc<Mutex<NFANode>>) -> Rc<Mutex<DFANode>>
    {
        let locked_node = node.lock().unwrap();
        let mut states: HashSet<i32> = HashSet::new();
        states.insert(locked_node.id);
        drop(locked_node);
        let mut nodes = Vec::new();
        nodes.push(node);
        return self.to_dfa_node(states, nodes);
    }

    fn single_nfa_to_dfa_node_raw(&mut self, node: Rc<Mutex<NFANode>>) -> *mut DFANode
    {
        let locked_node = node.lock().unwrap();
        let mut states: HashSet<i32> = HashSet::new();
        states.insert(locked_node.id);
        drop(locked_node);
        let mut nodes = Vec::new();
        nodes.push(node);
        return self.to_dfa_node_raw(states, nodes);
    }

    fn compute_dfa_node_id(node_ids: &HashSet<i32>) -> String
    {
        let mut id = String::new();
        for n_id in node_ids
        {
            let format = format!("{},", n_id);
            id.push_str(&format);
        }
        return id;
    }

    fn combine_dfa_nodes(&mut self, nodes: &Vec<Rc<Mutex<DFANode>>>) -> Rc<Mutex<DFANode>>
    {
        let mut ids: HashSet<i32> = HashSet::new();
        let mut nfa_nodes: Vec<Rc<Mutex<NFANode>>> = Vec::new();

        for item in nodes {
            // Lock it
            let lock = item.lock().unwrap();
            for sub_node in &lock.nodes {
                let locked_sub_node = sub_node.lock().unwrap();
                if !ids.insert(locked_sub_node.id) {
                    drop(locked_sub_node);
                    nfa_nodes.push(Rc::clone(sub_node));
                }
            }
        }

        return self.to_dfa_node(ids, nfa_nodes);
    }
}