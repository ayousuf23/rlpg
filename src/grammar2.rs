use std::{collections::{HashMap, HashSet, BTreeSet, BTreeMap}, fmt::Display};

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

impl Symbol {
   pub fn eof_symbol() -> Symbol
   {
        Symbol { name: "eof".to_string(), is_terminal: true }
   }
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub productions: Vec<*mut Production>,
}

#[derive(Debug, PartialEq)]
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

            if self.placeholder_index == (*self.production).prod.len() {
                result.push_str(".");
            }

            let end = format!(", {}", self.lookup_sym.name);
            result.push_str(&end)
        }
        return write!(f, "{}", result);
    }
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord, Clone)]
pub struct GrammarSet {
    pub set: BTreeSet<*mut LRItem>,
}

impl GrammarSet {
    pub fn new(set: BTreeSet<*mut LRItem>) -> GrammarSet {
        GrammarSet { set: set }
    }
}

#[derive(Debug, Clone)]
pub struct GrammarSetInfo {
    pub id: usize,
    pub transitions: HashMap<Symbol, usize>,
}

#[derive(Debug)]
pub enum Action {
    Shift(usize),
    // LHS and length of production
    Reduce(Symbol, usize),
    Accept
}

pub struct GrammarGenerator {
    rules: HashMap<Symbol, GrammarRule>,
    pub all_lr_items: HashMap<LRItem, *mut LRItem>,
    pub action_table: HashMap<(usize, Symbol), Action>,
    pub goto_table: HashMap<(usize, Symbol), usize>,
    pub non_terminals: HashSet<Symbol>,
    pub terminals: HashSet<Symbol>,
}

impl GrammarGenerator {

    pub fn new(terminals: HashSet<Symbol>) -> GrammarGenerator
    {
        GrammarGenerator { 
            rules: HashMap::new(), 
            all_lr_items: HashMap::new(),
            action_table: HashMap::new(),
            goto_table: HashMap::new(),
            non_terminals: HashSet::new(),
            terminals: terminals,
        }
    }

    pub fn add_rule(&mut self, symbol: Symbol, rule: GrammarRule)
    {
        self.non_terminals.insert(symbol.clone());
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
        sets.insert(cc_0.clone(), GrammarSetInfo { id: 0, transitions: HashMap::new() });

        let mut stack: Vec<GrammarSet> = Vec::new();
        
        stack.push(cc_0);

        // Used to give each set an ID
        let mut counter = 1;

        while let Some(front) = stack.pop() {
            let mut curr_info = sets.get_mut(&front).unwrap().clone();

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
                    sets.insert(temp.clone(), temp_info);
                    counter += 1;
                    stack.push(temp);
                }
            }

            sets.insert(front, curr_info);
        }

        return sets;
    }

    pub unsafe fn build_table(&mut self, cc: &HashMap<GrammarSet, GrammarSetInfo>)
    {
        for (key, value) in cc {

            // For each item in the set
            for item in &key.set {
                
                // Check for shift action
                if let Some(next_sym) = (**item).get_next_symbol() {
                    if let Some(reduce_dest) = value.transitions.get(&next_sym) {
                        self.action_table.insert((value.id, next_sym.clone()), Action::Shift(*reduce_dest));
                    }
                } 
                else if (**item).lhs.name == "root" && (**item).lookup_sym.name == "eof" {
                    self.action_table.insert((value.id, (**item).lookup_sym.clone()), Action::Accept);
                }
                else {
                    self.action_table.insert((value.id, (**item).lookup_sym.clone()), Action::Reduce((**item).lhs.clone(), (*(**item).production).prod.len()));
                }
            }

            // For each non-terminal
            for nt in &self.non_terminals {
                if let Some(dest) = value.transitions.get(nt) {
                    self.goto_table.insert((value.id, nt.clone()), *dest);
                }
            }
        }
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
    

    pub unsafe fn parse(&mut self, symbols: &Vec<Symbol>) -> bool {
        let mut stack: Vec<StackSymbol> = Vec::new();
        stack.push(StackSymbol::DollarSign);
        stack.push(StackSymbol::State(0));

        let mut word = symbols[0].clone();
        let mut word_index = 0;

        loop {
            let state = match &stack[stack.len() - 1] {
                StackSymbol::State(value) => *value,
                StackSymbol::Symbol(_) => panic!(),
                StackSymbol::DollarSign => panic!(),
            };
            //println!("state {}", state);
            let key = (state, word.clone());
            //println!("key {:?}", key);
            if let Some(action) = self.action_table.get(&key).clone() {
                //println!("{:?}", action);
                match action {
                    Action::Reduce(lhs, prod_len) => {
                        let num = 2 * prod_len;
                        for i in 0..num {
                            stack.pop();
                        }
                        let state = match &stack[stack.len() - 1] {
                            StackSymbol::State(value) => *value,
                            StackSymbol::Symbol(_) => panic!(),
                            StackSymbol::DollarSign => panic!(),
                        };
                        stack.push(StackSymbol::Symbol(lhs.clone()));
                        let goto = match self.goto_table.get(&(state, lhs.clone())) {
                            Some(value) => value,
                            None => panic!(),
                        };
                        stack.push(StackSymbol::State(*goto));
                        
                    },
                    Action::Shift(dest) => {
                        stack.push(StackSymbol::Symbol(word));
                        stack.push(StackSymbol::State(*dest));
                        word_index += 1;
                        word = symbols[word_index].clone();
                    },
                    Action::Accept => break,
                }

            }
            else {
                return false;
            }
        }
        return true;
    }
}

enum StackSymbol {
    Symbol(Symbol),
    State(usize),
    DollarSign,
}