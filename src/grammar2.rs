use std::{collections::{HashMap, HashSet, BTreeSet, BTreeMap}, fmt::Display};

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub productions: Vec<*mut Production>,
}

#[derive(Debug)]
pub struct Production {
    //pub lhs: Symbol,
    pub prod: Vec<Symbol>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct LRItem {
    pub production: *const Production,
    pub placeholder_index: usize,
    pub lookup_sym: Symbol,
    pub lhs: Symbol
}

impl LRItem {
    unsafe fn get_next_symbol(&self) -> Option<Symbol> {
        if self.placeholder_index >= (*self.production).prod.len()
        {
            return None;
        }

        return Some((*self.production).prod[self.placeholder_index].clone());
    }

    unsafe fn get_symbol_after_next_symbol(&self) -> Symbol {
        if self.placeholder_index + 1 >= (*self.production).prod.len()
        {
            return self.lookup_sym.clone();
        }

        return (*self.production).prod[self.placeholder_index+1].clone();
    }

    unsafe fn is_next_symbol(&self, symbol: &Symbol) -> bool
    {
        if let Some(next) = self.get_next_symbol() {
            return next == *symbol;
        }
        return false;
    }

    unsafe fn is_lookup_at_end(&self) -> bool {
        return self.placeholder_index >= (*self.production).prod.len();
    }

    unsafe fn get_lr_item_after_moving_lookup_index(&self) -> Option<LRItem> {
        if self.is_lookup_at_end() {
            return None;
        }
        let item = LRItem { 
            production: self.production, 
            placeholder_index: self.placeholder_index + 1, 
            lookup_sym: self.lookup_sym.clone(), 
            lhs: self.lhs.clone() 
        };
        return Some(item);
    }
}

impl Display for LRItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        unsafe {
            result = format!("{} -> ", self.lhs.name);

            for i in 0..(*self.production).prod.len() {
                if i == self.placeholder_index {
                    result.push_str(".");
                }

                result.push_str(&(*self.production).prod[i].name);

                if i != (*self.production).prod.len() - 1 {
                    result.push(' ');
                }
            }

            let end = format!(", {}", self.lookup_sym.name);
            result.push_str(&end)
        }
        return write!(f, "{}", result);
    }
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct GrammarSet {
    pub set: BTreeSet<*mut LRItem>,
}

impl GrammarSet {
    pub fn new(set: BTreeSet<*mut LRItem>) -> GrammarSet {
        GrammarSet { set: set }
    }
}

#[derive(Debug)]
pub struct GrammarSetInfo {
    pub id: usize,
    pub transitions: HashMap<Symbol, usize>,
}

pub struct GrammarGenerator {
    rules: HashMap<Symbol, GrammarRule>,
    transitions: HashMap<*const GrammarSet, Vec<(Symbol, *const GrammarSet)>>,
    pub all_lr_items: HashMap<LRItem, *mut LRItem>,
}

impl GrammarGenerator {

    pub fn new() -> GrammarGenerator
    {
        GrammarGenerator { 
            rules: HashMap::new(), 
            transitions: HashMap::new(), 
            all_lr_items: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, symbol: Symbol, rule: GrammarRule)
    {
        self.rules.insert(symbol, rule);
    }

    // Function to compute first set
    pub fn get_first_set(&self, symbol: &Symbol) -> HashSet<Symbol>
    {
        // Clone string to create the stack
        let mut stack: Vec<Symbol> = Vec::new();
        stack.push(symbol.clone());
        let mut seen: HashSet<Symbol> = HashSet::new();

        // Set of symbols in first set
        let mut set: HashSet<Symbol> = HashSet::new();

        while !stack.is_empty() {
            let front = stack.remove(0);

            if !seen.insert(front.clone()) {
                continue;
            }

            if front.is_terminal {
                set.insert(front.clone());
            }
            else {
                if let Some(rule) = self.rules.get(&front) {
                    // Run get first set on the productions
                    for prod in &rule.productions {
                        unsafe {
                            let prod_first_sym = &(**prod).prod[0];
                            stack.push(prod_first_sym.clone());
                        }
                    }
                }
                else {
                    // Throw an error here
                    todo!();
                }
            }
        }
        return set;
    }

    pub fn get_closure(&mut self, set: GrammarSet) -> GrammarSet
    {   
        // Keep track of the items that are done
        let mut done = BTreeSet::new();

        // Keep a stack of the items that need to be dealt with
        let mut stack: Vec<*mut LRItem> = Vec::new();

        // Add items from set to stack
        for item in &(set).set {
            stack.push(*item);
        }

        // Resulting set
        let mut result = set;

        unsafe {
            // Get the LRItem at th front of the stack
            while let Some(lr_item) = stack.pop() {
                if (*lr_item).is_lookup_at_end() {
                    continue;
                }

                // Add this lr_item to the done set
                if !done.insert(lr_item) {
                    continue;
                }

                // Get the next symbol
                let next_sym = (*lr_item).get_next_symbol().unwrap();

                // Go to the next lr_item if the next symbol is a terminal
                if next_sym.is_terminal {
                    continue;
                }

                // Get the first set for this symbol
                let sym_after_next_sym = (*lr_item).get_symbol_after_next_symbol();
                let first_set = self.get_first_set(&sym_after_next_sym);

                //println!("sym: {}, first set: {:?}", next_sym.name, first_set);

                let rule_for_next = match self.rules.get(&next_sym) {
                    Some(value) => value,
                    None => todo!(),
                }.clone();
                
                // Go through the possible productions
                for production in &rule_for_next.productions {
                    // Go through possible lookup symbols
                    for lookup_sym in &first_set {
                        let lr_item = self.get_lr_item_from_prod(*production, next_sym.clone(), lookup_sym.clone());
                        // Insert into stack
                        stack.push(lr_item);
                        result.set.insert(lr_item);
                    }
                }

                
            }
        }

        return result;
    }

    pub fn get_goto(&mut self, set: &GrammarSet, symbol: &Symbol) -> GrammarSet
    {
        let mut result = GrammarSet::new(BTreeSet::new());
        for item in &set.set {
            unsafe {
                if (**item).is_next_symbol(symbol) {
                    let moved_item = (**item).get_lr_item_after_moving_lookup_index().unwrap();
                    let moved_item = self.add_lr_item_or_get_existing(moved_item);
                    result.set.insert(moved_item);
                }
            }
        }
        return self.get_closure(result);
    }

    pub fn build_cannocial_collection(&mut self) -> HashMap<GrammarSet, GrammarSetInfo>
    {
        let mut sets: HashMap<GrammarSet, GrammarSetInfo> = HashMap::new();
        let cc_0 = self.get_goal_grammar_set();
        let cc_0 = self.get_closure(cc_0);
        sets.insert(cc_0, GrammarSetInfo { id: 0, transitions: HashMap::new() });

        let mut stack: Vec<GrammarSet> = Vec::new();
        
        stack.push(cc_0);

        // Used to give each set an ID
        let mut counter = 1;

        while let Some(front) = stack.pop() {
            let curr_set_id = counter;
            let curr_info = sets.get(&front).unwrap();

            let syms_after_placeholder = self.get_symbols_after_placeholder(&front);
            for sym in syms_after_placeholder {
                let temp = self.get_goto(&front, &sym);
                // Check if temp is in sets
                if let Some(info) = sets.get(&temp) {
                    // Record transition
                    curr_info.transitions.insert(sym.clone(), info.id);
                }
                else {
                    // Record transition
                    let temp_info = GrammarSetInfo {id: counter, transitions: HashMap::new()};
                    curr_info.transitions.insert(sym.clone(), counter);

                    // Insert temp into sets and stack
                    sets.insert(temp, temp_info);
                    counter += 1;
                    stack.push(temp);
                }
            }
        }

        return sets;
    }

    fn get_symbols_after_placeholder(&mut self, set: &GrammarSet) -> HashSet<Symbol>
    {
        let mut symbols = HashSet::new();
        for lr_item in &set.set {
            unsafe {
                if let Some(next) = (**lr_item).get_next_symbol() {
                    symbols.insert(next);
                }
            }
        }
        return symbols;
    }

    fn add_lr_item_or_get_existing(&mut self, item: LRItem) -> *mut LRItem
    {
        if let Some(value) = self.all_lr_items.get(&item)
        {
            return *value;
        }
        else {
            let item_pointer = Box::into_raw(Box::new(item.clone()));
            self.all_lr_items.insert(item, item_pointer);
            return item_pointer;
        }
    }

    fn get_lr_item_from_prod(&mut self, prod: *const Production, lhs: Symbol, lookup: Symbol) -> *mut LRItem
    {
        let lr_item = LRItem {
            production: prod,
            placeholder_index: 0,
            lookup_sym: lookup,
            lhs,
        };
        return self.add_lr_item_or_get_existing(lr_item);
    }

    pub fn get_goal_grammar_set(&mut self) -> GrammarSet {
        let root_sym = Symbol { name: "root".to_string(), is_terminal: false };
        let root_rule = match self.rules.get(&root_sym) {
            Some(value) => value,
            None => todo!(),
        }.clone();

        let mut grammar_set = GrammarSet::new(BTreeSet::new());
        let eof_sym = Symbol { name: "eof".to_string(), is_terminal: true };
        
        for prod in &root_rule.productions {
            // Convert to LR Item
            let lr_item = self.get_lr_item_from_prod(*prod, root_sym.clone(), eof_sym.clone());
            grammar_set.set.insert(lr_item);
        }

        return grammar_set;
    }
}