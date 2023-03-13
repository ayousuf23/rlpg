struct CodeGen;

impl CodeGen {

    fn create_transition_function()
    {
        let header = "fn transition(curr: i32, trans: TransitionKind)";
        let template = "if curr == 1 { \
            if let TransitionKind::Character(trans_char) = trans {
                if trans_char == 'x'
            } else {
                // Do any char 
            }
        }
            ";
    }
}