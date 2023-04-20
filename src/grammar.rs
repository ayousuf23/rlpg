use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

#[derive(Debug)]
pub struct GrammarRule {
    pub name: String,
    pub productions: Vec<*mut Production>,
}

#[derive(Debug)]
pub struct Production {
    pub lhs: Symbol,
    pub prod: Vec<Symbol>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct LRItem {
    pub production: *const Production,
    pub placeholder_index: usize,
    pub lookup_sym: Symbol,
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct GrammarSet {
    pub set: BTreeSet<LRItem>,
}

impl GrammarSet {
    pub fn new(set: BTreeSet<LRItem>) -> GrammarSet {
        GrammarSet { set: set }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Action {
    Shift,
    Reduce(Symbol, *const Production),
    Accept,
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

    unsafe fn is_lookup_at_end(&self) -> bool {
        return self.placeholder_index >= (*self.production).prod.len();
    }
}

pub struct GrammarGenerator {
    rules: HashMap<Symbol, GrammarRule>,
    transitions: HashMap<*const GrammarSet, Vec<(Symbol, *const GrammarSet)>>,
    pub action_table: BTreeMap<(i32, Symbol), Action>,
    pub sets: HashSet<*mut GrammarSet>,
}

impl GrammarGenerator {

    pub fn new() -> GrammarGenerator
    {
        GrammarGenerator { 
            rules: HashMap::new(), 
            transitions: HashMap::new(), 
            action_table: BTreeMap::new(),
            sets: HashSet::new(),
        }
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
    pub fn get_closure(&mut self, set: *mut GrammarSet) -> *mut GrammarSet
    {   
        // Keep track of the items that are done
        let mut done = BTreeSet::new();

        // Keep a stack of the items that need to be dealt with
        let mut stack: Vec<LRItem> = Vec::new();

        unsafe {
            // Add items from set to stack
            for item in &(*set).set {
                stack.push(item.clone());
            }

            // Main loop
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
                //println!("{:?}", next_sym);

                // Go to the next lr_item if the next symbol is a terminal
                if next_sym.is_terminal {
                    continue;
                }

                //println!("hello");
                //println!("{:?}", next_sym);
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
                //println!("{:?}", syms_after_next_sym);
                
                // Get the first set of the above symbols
                let mut first_set_of_syms_after_next_sym = HashSet::new();
                self.get_first_set(&syms_after_next_sym, &mut first_set_of_syms_after_next_sym);
                println!("{:?}", first_set_of_syms_after_next_sym);
                
                // Go through the possible productions
                for production in &rule.productions {
                    // Go through possible lookup symbols
                    println!("{:?}", **production);
                    for lookup_sym in &first_set_of_syms_after_next_sym {
                        let lr_item = self.get_lr_item_from_prod(*production as *const Production, lookup_sym.clone());
                        // Insert into stack
                        stack.push(lr_item);
                    }
                }
            }
        }

        let result_grammar_set = GrammarSet {set: done};
        if let Some(already_set) = self.get_set_already_contained(&result_grammar_set) {
            return already_set;
        }
        let pointer = Box::into_raw(Box::new(result_grammar_set));
        self.sets.insert(pointer);
        return pointer;
    }

    unsafe fn get_goto(&mut self, set: *mut GrammarSet, symbol: Symbol) -> *mut GrammarSet
    {
        let mut moved = GrammarSet {set: BTreeSet::new()};
        for item in &(*set).set {
            if item.is_next_symbol(&symbol) {
                let moved_forward = LRItem {
                    production: item.production,
                    placeholder_index: item.placeholder_index + 1,
                    lookup_sym: item.lookup_sym.clone()
                };
                moved.set.insert(moved_forward);
            }
        }

        // Return closure of moved
        return self.get_closure(Box::into_raw(Box::new(moved)));
    }

    pub unsafe fn build_cannocial_collection(&mut self) -> BTreeSet<*mut GrammarSet>
    {
        let mut seen: BTreeSet<*mut GrammarSet> = BTreeSet::new();
        // Get goal grammar set
        let goal = self.get_goal_grammar_set();

        GrammarGenerator::print(&*goal);

        let cc0 = self.get_closure(goal);

        /*let mut stack: BTreeSet<*mut GrammarSet> = BTreeSet::new();
        stack.insert(cc0);

        // Mark processed sets here
        

        while !stack.is_empty() {
            let set = stack.pop_first().unwrap();

            // Skip the set if it has been seen already
            if seen.contains(&set) {
                continue;
            }

            // For each x following . in an item in CC
            let mut x_set: BTreeSet<Symbol> = BTreeSet::new();
            for subset in &(*set).set {
                unsafe {
                    if let Some(next_sym) = subset.get_next_symbol() {
                        x_set.insert(next_sym);
                    }
                }
            }

            for x in x_set {
                let temp = self.get_goto(set, x.clone());
                
                // Destination set
                let destination_set: *const GrammarSet;

                // Check if this set was already seen
                if let Some(temp_already_seen) = seen.get(&temp) {
                    // Add transition from set to temp_already_seen
                    destination_set = &**temp_already_seen as *const GrammarSet;
                }
                else {
                    // Add temp to stack if not already on stack
                    if let Some(temp_stack) = stack.get(&temp) {
                        // Add transition from set to temp_stack
                        destination_set = &**temp_stack as *const GrammarSet;
                    } else {
                        destination_set = &*temp as *const GrammarSet;
                        stack.insert(temp);
                    }
                }

                // Record transition from cc_i to temp on x
                self.add_transition(&*set as *const GrammarSet, x, destination_set);              
            }

            // Insert into seen (mark set)
            seen.insert(set);
        }*/
        return seen;
    }

    pub unsafe fn fill_table(&mut self, goal: GrammarSet)
    {
        // Build canonical collection
        let cc = self.build_cannocial_collection();

        let mut cc_count = 0;

        for cc_i in cc {
            // For each item I in cc_i
            for i in &(*cc_i).set {

                if self.is_acceptable(&i) {
                    //self.action_table.insert((cc_count, i.lookup_sym), Action::Accept);
                }
                else if i.is_lookup_at_end() {
                    // Add reduction action
                    let reduce_action = Action::Reduce((*i.production).lhs.clone(), i.production);
                    //self.action_table.insert((cc_count, i.lookup_sym), reduce_action);
                }
            }

            cc_count += 1;
        }
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

    fn add_transition(&mut self, source_set: *const GrammarSet, symbol: Symbol, dest_set: *const GrammarSet)
    {
        if let Some(trans) = self.transitions.get(&source_set) {
            //trans.push((symbol, dest_set));
        } else {
            let new_vec = vec![(symbol, dest_set)];
            self.transitions.insert(source_set, new_vec);
        }
    }

    fn is_acceptable(&self, item: &LRItem) -> bool
    {
        return false;
    }

    fn get_goal_grammar_set(&mut self) -> *mut GrammarSet {
        let root_rule = self.rules.get(&Symbol { name: "root".to_string(), is_terminal: false });
        if root_rule.is_none() {
            // Throw an error
            panic!();
        }
        let root_rule = root_rule.unwrap();

        let mut grammar_set: BTreeSet<LRItem> = BTreeSet::new();
        for prod in &root_rule.productions {
            // Convert to LR Item
            let lr_item = self.get_lr_item_from_prod(*prod, Symbol { name: "eof".to_string(), is_terminal: true });
            grammar_set.insert(lr_item);
        }

        return self.insert_set_or_get_already_contained(GrammarSet { set: grammar_set });
    }

    fn insert_set_or_get_already_contained(&mut self, set: GrammarSet) -> *mut GrammarSet
    {
        if let Some(already_contained) = self.get_set_already_contained(&set)
        {
            return already_contained;
        }
        let result = Box::into_raw(Box::new(set));
        self.sets.insert(result);
        return result;
    }

    fn get_set_already_contained(&self, set: &GrammarSet) -> Option<*mut GrammarSet>
    {
        for contained_set in &self.sets {
            unsafe {
                if **contained_set == *set {
                    return Some(*contained_set);
                }
            }
        }
        return None;
    }

    unsafe fn print(grammar: &GrammarSet)
    {
        for item in &grammar.set {
            println!("{:?}", *item.production);
        }
    }

}