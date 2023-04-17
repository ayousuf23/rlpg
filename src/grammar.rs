use std::collections::{HashMap, HashSet, BTreeSet};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

pub struct GrammarRule {
    pub name: String,
    pub productions: Vec<*mut Production>,
}

#[derive(Debug)]
pub struct Production {
    pub prod: Vec<Symbol>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct LRItem {
    pub production: *const Production,
    pub placeholder_index: usize,
    pub lookup_sym: Symbol,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct GrammarSet {
    pub set: BTreeSet<LRItem>
}

pub enum Action {
    Shift,
    Reduce,
    Accept
}

impl LRItem {
    unsafe fn get_next_symbol(&self) -> Option<Symbol> {
        if self.placeholder_index >= (*self.production).prod.len()
        {
            return None;
        }

        return Some((*self.production).prod[self.placeholder_index].clone());
    }

    unsafe fn is_next_symbol(&self, symbol: &Symbol) -> bool
    {
        if let Some(next) = self.get_next_symbol() {
            return next == *symbol;
        }
        return false;
    }
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
    pub fn get_closure(&self, set: &HashSet<LRItem>) -> HashSet<LRItem>
    {   
        // Keep track of the items that are done
        let mut done = HashSet::new();

        // Keep a stack of the items that need to be dealt with
        let mut stack: Vec<LRItem> = Vec::new();

        // Add items from set to stack
        for item in set {
            stack.push(item.clone());
        }

        // Main loop
        unsafe {
            while let Some(lr_item) = stack.pop() {
                if lr_item.placeholder_index >= (*lr_item.production).prod.len() {
                    continue;
                }

                // Add this lr_item to the done set
                if !done.insert(lr_item.clone()) {
                    continue;
                }

                // Get the next symbol
                let next_sym = (*lr_item.production).prod[lr_item.placeholder_index].clone();
                println!("{:?}", next_sym);

                // Go to the next lr_item if the next symbol is a terminal
                if next_sym.is_terminal {
                    continue;
                }

                println!("hello");

                // Get the grammar rule associated with next_sym
                let rule = match self.rules.get(&next_sym) {
                    Some(val) => val,
                    None => {
                        // Throw an error
                        todo!();
                    }
                };

                // Get the symbols after next_sym
                let syms_after_next_sym = self.get_next_symbols(&lr_item, &lr_item.placeholder_index + 1);
                println!("{:?}", syms_after_next_sym);
                // Get the first set of the above symbols
                let mut first_set_of_syms_after_next_sym = HashSet::new();
                self.get_first_set(&syms_after_next_sym, &mut first_set_of_syms_after_next_sym);
                println!("{:?}", first_set_of_syms_after_next_sym);
                
                // Go through the possible productions
                for production in &rule.productions {
                    // Go through possible lookup symbols
                    for lookup_sym in &first_set_of_syms_after_next_sym {
                        let lr_item = self.get_lr_item_from_prod(*production as *const Production, lookup_sym.clone());
                        // Insert into stack
                        stack.push(lr_item);
                    }
                }
            }
        }
        return done;
    }

    fn get_goto(&self, set: &HashSet<LRItem>, symbol: Symbol) -> HashSet<LRItem>
    {
        let mut moved = HashSet::new();
        for item in set {
            unsafe {
                if item.is_next_symbol(&symbol) {
                    let moved_forward = LRItem {
                        production: item.production,
                        placeholder_index: item.placeholder_index + 1,
                        lookup_sym: item.lookup_sym.clone()
                    };
                    moved.insert(moved_forward);
                }
            }
        }

        // Return closure of moved
        return self.get_closure(&moved);
    }

    fn build_cannocial_collection(&self, goal: GrammarSet)
    {
        let cc0 = self.get_closure(&goal.set);

        let mut stack: Vec<HashSet<LRItem>> = Vec::new();
        stack.push(cc0);

        // Mark processed sets here
        let mut seen: HashSet<>
    }

    // Get sequence of symbols after a certain index in an LR item
    fn get_next_symbols(&self, lr_item: &LRItem, index: usize) -> Vec<Symbol>
    {   
        let mut result = Vec::new();

        unsafe {
            let size = (*lr_item.production).prod.len();
            for i in index..size
            {
                result.push((*lr_item.production).prod[i].clone());
            }
        }

        // Add lookup symbol
        result.push(lr_item.lookup_sym.clone());

        return result;
    }

    fn get_lr_item_from_prod(&self, prod: *const Production, lookup_sym: Symbol) -> LRItem
    {
        LRItem { production: prod, placeholder_index: 0, lookup_sym }
    }

}