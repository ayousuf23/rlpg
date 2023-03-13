use std::collections::{HashMap, HashSet};

use crate::{nfa::TransitionKind, dfa_builder::{DFANode, DFANodeKind}};

pub struct TableDFA {
    pub transitions: HashMap<(i32, TransitionKind), i32>,
    pub accepting_states: HashMap<i32, String>,
}

impl TableDFA {
    pub fn new() -> TableDFA
    {
        TableDFA {
            transitions: HashMap::new(),
            accepting_states: HashMap::new(),
        }
    }
}

pub struct TableDFABuilder {
    mapping: HashMap<*const DFANode, i32>,
    node_counter: i32,
}

impl TableDFABuilder {

    pub unsafe fn build_table_dfa(&mut self, root: *const DFANode) -> TableDFA
    {
        let mut table = TableDFA::new();

        let mut stack: Vec<*const DFANode> = Vec::new();
        stack.push(root);
        let mut seen: HashSet<*const DFANode> = HashSet::new();

        while let Some(node) = stack.pop()
        {
            if !seen.insert(node)
            {
                continue;
            }

            // Get the node's id
            let id: i32 = self.get_node_id(node);

            // Check if this is an accepting node
            if let DFANodeKind::Accept(token) = &(*node).kind {
                table.accepting_states.insert(id, token.to_string());
            }

            for (trans_kind, dest) in &(*node).raw_transitions
            {
                let dest_id = self.get_node_id(*dest);
                // Add a transition to the table
                table.transitions.insert((id, trans_kind.clone()), dest_id);

                // Add destination to stack
                stack.push(*dest);
            }
        }

        return table;
    }

    fn get_node_id(&mut self, node: *const DFANode) -> i32
    {
        return match self.mapping.get(&node)
        {
            Some(value) => *value,
            None => {
                self.mapping.insert(node, self.node_counter);
                self.node_counter += 1;
                self.node_counter - 1
            }
        };
    }
}