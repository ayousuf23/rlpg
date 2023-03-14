pub enum TransitionKind {
    Character(char),

            AnyChar,

    }
fn is_accepting(state: i32) -> Option<String>
{
    return match state {
            7 => Some("char".to_string()),
            3 => Some("char".to_string()),
            5 => Some("char".to_string()),
            9 => Some("char".to_string()),
            11 => Some("char".to_string()),
            13 => Some("char".to_string()),
            15 => Some("char".to_string()),
            17 => Some("char".to_string()),
            _ => None
    }
}
fn transition(curr: i32, trans: TransitionKind) -> i32
{
    if curr == 6
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 7;
                    }
            }
    }
    if curr == 9
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 10;
                    }
            }
    }
    if curr == 17
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 6;
                    }
            }
    }
    if curr == 15
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 16;
                    }
            }
    }
    if curr == 10
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 11;
                    }
            }
    }
    if curr == 16
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 17;
                    }
            }
    }
    if curr == 12
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 13;
                    }
            }
    }
    if curr == 13
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 14;
                    }
            }
    }
    if curr == 5
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 6;
                    }
            }
    }
    if curr == 8
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 9;
                    }
            }
    }
    if curr == 7
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 8;
                    }
            }
    }
    if curr == 11
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 12;
                    }
            }
    }
    if curr == 4
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 5;
                    }
            }
    }
    if curr == 14
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 15;
                    }
            }
    }
    if curr == 2
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'i'
                    {
                            return 3;
                    }
            }
    }
    if curr == 1
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 2;
                    }
            }
    }
    if curr == 3
    {
            if let TransitionKind::Character(trans_char) = trans
            {
                    if trans_char == 'h'
                    {
                            return 4;
                    }
            }
    }
    panic!();
}
pub fn parse(text: String) -> Vec<String> 
    {
        let mut curr_state = 1;
        let seq: Vec<char> = text.chars().collect();
        let mut index = 0;
        let mut tokens: Vec<String> = Vec::new();
        while index < seq.len() {
            // Check if accepting state
            if let Some(token) = is_accepting(curr_state)
            {
                tokens.push(token);
            }
            
            // Perform transition or error
            let trans_kind = TransitionKind::Character(seq[index]);
            curr_state = transition(curr_state, trans_kind);
            index += 1;
        }
        if let Some(token) = is_accepting(curr_state)
        {
            tokens.push(token);
        }
        tokens
    }

pub fn main()
{
    println!("Enter a string to match: ");
    let mut to_check = String::new();
    std::io::stdin().read_line(&mut to_check).expect("failed to readline");
    let to_check = to_check.trim().to_string();
    let result = parse(to_check);
    println!("{:?}", result);
}