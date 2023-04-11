use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

pub struct GrammarRule {
    pub name: String,
    pub productions: Vec<*mut Production>,
}

pub struct Production {
    pub prod: Vec<Symbol>,
}

struct LRItem {
    production: *const Production,
    placeholder_index: usize,
}

pub struct GrammarGenerator {
    rules: HashMap<Symbol, GrammarRule>,
}

impl GrammarGenerator {

    pub fn new() -> GrammarGenerator
    {
        GrammarGenerator { rules: HashMap::new() }
    }

    pub fn add_rule(&mut self, symbol: Symbol, rule: GrammarRule)
    {
        self.rules.insert(symbol, rule);
    }

    // Function to compute first set
    pub fn get_first_set(&self, string: &Vec<Symbol>, set: &mut HashSet<Symbol>)
    {
        for token in string {
            if token.is_terminal {
                set.insert(token.clone());
            }
            else {
                // Get production for symbol
                if let Some(rule) = self.rules.get(token) {
                    // Run get first set on the productions
                    for prod in &rule.productions {
                        unsafe {
                            self.get_first_set(&(**prod).prod, set);
                        }
                    }
                }
                else {
                    // Throw an error here
                    todo!();
                }
            }
        }
    }

    // Function to compute closure
}