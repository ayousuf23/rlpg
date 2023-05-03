use std::collections::HashMap;


#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)] pub struct
Symbol { pub name : String, pub is_terminal : bool, } impl Symbol
{
    pub fn eof_symbol() -> Symbol
    { Symbol { name : "eof".to_string(), is_terminal : true } }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug, PartialOrd, Ord)] pub struct Token
{
    pub lexeme : String, pub line : usize, pub start_col : usize, pub end_col
    : usize, pub symbol : Symbol,
} impl Token
{
    pub fn
    new(lexeme : String, start_col : usize, end_col : usize, symbol : Symbol)
    -> Token
    {
        Token
        {
            lexeme : lexeme, line : 0, start_col : start_col, end_col :
            end_col, symbol : symbol
        }
    }
}
#[derive(Debug)] pub enum Action
{ Shift(usize), Reduce(Symbol, usize), Accept }
enum StackSymbol { Symbol(Symbol), State(usize), DollarSign, }
#[derive(Debug)] pub struct TreeNode
{ pub token : Token, pub children : Vec < TreeNode >, }
#[derive(Debug)] pub enum ErrorKind
{ GrammarParseFailed, TokenizationFailed(usize, usize), } impl ErrorKind
{
    pub fn get_err_message(& self) -> String
    {
        return match self
        {
            Self :: GrammarParseFailed =>
            "Error: the token sequence is not accepted by the grammar".to_string(),
            Self :: TokenizationFailed(start, end) => format!
            ("Error: unable to tokenize the sequence of characters starting at {} and ending at {}",
            start, end),
        }
    }
}

pub enum TransitionKind { Character(char), AnyChar, }
fn is_accepting(state: i32) -> Option<String>
{
	return match state {
		8 => Some("number".to_string()),
		11 => Some("number".to_string()),
		3 => Some("number".to_string()),
		7 => Some("number".to_string()),
		10 => Some("number".to_string()),
		12 => Some("plus".to_string()),
		13 => Some("number".to_string()),
		6 => Some("minus".to_string()),
		15 => Some("number".to_string()),
		2 => Some("divide".to_string()),
		4 => Some("number".to_string()),
		14 => Some("number".to_string()),
		9 => Some("times".to_string()),
		5 => Some("number".to_string()),
		_ => None
	}
}
fn transition(curr: i32, trans: TransitionKind) -> Option<i32>
{
	if curr == 1
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '/'
 			{
 				return Some(2);
			}
			if trans_char == '*'
 			{
 				return Some(9);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '-'
 			{
 				return Some(6);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '+'
 			{
 				return Some(12);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
		}
	}
	if curr == 13
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
		}
	}
	if curr == 15
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
		}
	}
	if curr == 7
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
		}
	}
	if curr == 8
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
		}
	}
	if curr == 14
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
		}
	}
	if curr == 4
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
		}
	}
	if curr == 11
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
		}
	}
	if curr == 5
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
		}
	}
	if curr == 3
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '3'
 			{
 				return Some(8);
			}
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
		}
	}
	if curr == 10
	{
		if let TransitionKind::Character(trans_char) = trans
		{
			if trans_char == '0'
 			{
 				return Some(15);
			}
			if trans_char == '9'
 			{
 				return Some(4);
			}
			if trans_char == '8'
 			{
 				return Some(13);
			}
			if trans_char == '5'
 			{
 				return Some(10);
			}
			if trans_char == '7'
 			{
 				return Some(3);
			}
			if trans_char == '1'
 			{
 				return Some(11);
			}
			if trans_char == '6'
 			{
 				return Some(5);
			}
			if trans_char == '2'
 			{
 				return Some(14);
			}
			if trans_char == '4'
 			{
 				return Some(7);
			}
			if trans_char == '3'
 			{
 				return Some(8);
			}
		}
	}
	return None;
}

fn get_action_table() -> HashMap<(usize, Symbol), Action> {
return HashMap::from([
	((1, Symbol {name: "divide".to_string(), is_terminal: true }), Action::Shift(14)),
	((7, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "expression".to_string(), is_terminal: false}, 3)),
	((8, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(11)),
	((2, Symbol {name: "minus".to_string(), is_terminal: true }), Action::Shift(4)),
	((2, Symbol {name: "plus".to_string(), is_terminal: true }), Action::Shift(5)),
	((1, Symbol {name: "times".to_string(), is_terminal: true }), Action::Shift(13)),
	((4, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(6)),
	((1, Symbol {name: "minus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 1)),
	((6, Symbol {name: "divide".to_string(), is_terminal: true }), Action::Shift(9)),
	((16, Symbol {name: "plus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((0, Symbol {name: "term".to_string(), is_terminal: false }), Action::Shift(2)),
	((3, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Accept),
	((0, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(1)),
	((9, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(10)),
	((14, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(15)),
	((15, Symbol {name: "minus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((0, Symbol {name: "expression".to_string(), is_terminal: false }), Action::Shift(3)),
	((16, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((1, Symbol {name: "plus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 1)),
	((11, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((15, Symbol {name: "plus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((16, Symbol {name: "minus".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((2, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "expression".to_string(), is_terminal: false}, 1)),
	((13, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(16)),
	((4, Symbol {name: "term".to_string(), is_terminal: false }), Action::Shift(12)),
	((5, Symbol {name: "number".to_string(), is_terminal: true }), Action::Shift(6)),
	((12, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "expression".to_string(), is_terminal: false}, 3)),
	((10, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((1, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 1)),
	((5, Symbol {name: "term".to_string(), is_terminal: false }), Action::Shift(7)),
	((15, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 3)),
	((6, Symbol {name: "times".to_string(), is_terminal: true }), Action::Shift(8)),
	((6, Symbol {name: "eof".to_string(), is_terminal: true }), Action::Reduce(Symbol {name: "term".to_string(), is_terminal: false}, 1)),
]);
}

fn get_goto_table() -> HashMap<(usize, Symbol), usize> {
return HashMap::from([
((5, Symbol {name: "term".to_string(), is_terminal: false }), 7),
((4, Symbol {name: "term".to_string(), is_terminal: false }), 12),
((0, Symbol {name: "expression".to_string(), is_terminal: false }), 3),
((0, Symbol {name: "term".to_string(), is_terminal: false }), 2),
]);
}

pub fn get_tokens(text : String) -> Result < Vec < Token >, ErrorKind >
{
    let mut curr_state = 1 ; let seq : Vec < char > = text.chars().collect() ;
    let mut index = 0 ; let mut tokens : Vec < Token > = Vec :: new() ; let
    mut start_col = 0 ; let mut end_col = 0 ; while index < seq.len()
    {
        let trans_kind = TransitionKind :: Character(seq [index]) ; if let
        Some(next_state) = transition(curr_state, trans_kind)
        { curr_state = next_state ; end_col += 1 ; } else
        {
            if let Some(token) = is_accepting(curr_state)
            {
                if! token.is_empty()
                {
                    let sym = Symbol
                    { name : token.to_string(), is_terminal : true } ; let
                    lexeme = text [start_col .. end_col].to_string() ; let token
                    = Token :: new(lexeme, start_col, end_col - 1, sym) ;
                    tokens.push(token) ;
                } curr_state = 1 ; start_col = index ; end_col = index ; index
                -= 1 ;
            } else
            {
                return
                Err(ErrorKind :: TokenizationFailed(start_col, end_col)) ;
            }
        } index += 1 ;
    } if let Some(token) = is_accepting(curr_state)
    {
        if! token.is_empty()
        {
            let sym = Symbol { name : token.to_string(), is_terminal : true }
            ; let lexeme = text [start_col .. end_col].to_string() ; let token
            = Token :: new(lexeme, start_col, end_col - 1, sym) ;
            tokens.push(token) ;
        }
    } let eof_token = Token ::
    new("eof".to_string(), end_col, end_col, Symbol :: eof_symbol()) ;
    tokens.push(eof_token) ; Ok(tokens)
}
pub fn parse(symbols : & Vec < Token >) -> Result < TreeNode, ErrorKind >
{
    let action_table = get_action_table() ; let goto_table : HashMap <
    (usize, Symbol), usize > = get_goto_table() ; let mut stack : Vec <
    StackSymbol > = Vec :: new() ; stack.push(StackSymbol :: DollarSign) ;
    stack.push(StackSymbol :: State(0)) ; let mut word = symbols
    [0].symbol.clone() ; let mut word_index = 0 ; let mut node_children = vec!
    [TreeNode { token : symbols [0].clone(), children : Vec :: new() }] ; let
    mut start_col = 0 ; let mut end_col = 0 ; loop
    {
        let state = match & stack [stack.len() - 1]
        {
            StackSymbol :: State(value) => * value, StackSymbol :: Symbol(_)
            => panic! (), StackSymbol :: DollarSign => panic! (),
        } ; let key = (state, word.clone()) ; if let Some(action) =
        action_table.get(& key).clone()
        {
            match action
            {
                Action :: Reduce(lhs, prod_len) =>
                {
                    let num = 2 * prod_len ; for i in 0 .. num { stack.pop() ; }
                    let state = match & stack [stack.len() - 1]
                    {
                        StackSymbol :: State(value) => * value, StackSymbol ::
                        Symbol(_) => panic! (), StackSymbol :: DollarSign => panic!
                        (),
                    } ; stack.push(StackSymbol :: Symbol(lhs.clone())) ; let
                    goto = match goto_table.get(& (state, lhs.clone()))
                    { Some(value) => value, None => panic! (), } ;
                    stack.push(StackSymbol :: State(* goto)) ; let token = Token
                    :: new(lhs.name.to_string(), 0, 0, lhs.clone()) ; let node =
                    TreeNode { token : token, children : node_children } ;
                    node_children = vec! [node] ;
                }, Action :: Shift(dest) =>
                {
                    stack.push(StackSymbol :: Symbol(word)) ;
                    stack.push(StackSymbol :: State(* dest)) ; word_index += 1 ;
                    word = symbols [word_index].symbol.clone() ; let token =
                    Token ::
                    new(symbols [word_index].lexeme.to_string(), symbols
                    [word_index].start_col, symbols [word_index].end_col,
                    word.clone()) ; let node = TreeNode
                    { token : token, children : Vec :: new() } ;
                    node_children.push(node) ;
                }, Action :: Accept => break,
            }
        } else { return Err(ErrorKind :: GrammarParseFailed) ; }
    } let root_node = TreeNode
    {
        token : Token ::
        new("root".to_string(), 0, 0, Symbol
        { name : "root".to_string(), is_terminal : false }), children :
        node_children
    } ; return Ok(root_node) ;
}
pub fn main()
        {{
            println!("Enter a string to match: ");
            let mut to_check = String::new();
            std::io::stdin().read_line(&mut to_check).expect("failed to readline");
            let to_check = to_check.trim().to_string();
            let result = get_tokens(to_check);
            //println!("{:?}", result);
            if let Err(err) = result {
                println!("Error");
                return;
            }

            let mut result = result.unwrap();
            let eof_token = Token::new("eof".to_string(), 0, 0, Symbol::eof_symbol());
            result.push(eof_token);

        let grammar_result = parse(&result);println!("Result: {}", grammar_result.is_err())}}