mod output;
use output::TreeNode;

fn recurse_tree(node: &TreeNode) -> f32
{
    let name = &node.token.symbol.name;
    if name == "number" {
        // Check num children
        return node.token.lexeme.parse().unwrap();
    }
    else if name == "unary_expr" {
        return unary_expr(node);
    }
    else if name == "primary_expr" {
        return primary_expr(node);
    }
    else if name == "term" {
        return term(node);
    }
    else if name == "expression" {
        return expression(node);
    }
    
    for child in &node.children {
        return recurse_tree(child);
    }

    return 0.0;
}

fn primary_expr(node: &TreeNode) -> f32 {
    return recurse_tree(&node.children[0]);
}

fn unary_expr(node: &TreeNode) -> f32 {
    return recurse_tree(&node.children[0]);
}

fn term(node: &TreeNode) -> f32 {
    return recurse_tree(&node.children[0]);
}

fn expression(node: &TreeNode) -> f32
{
    let op1 = recurse_tree(&node.children[0]);
    if node.children.len() == 1 {
        return op1;
    }

    let op2 = recurse_tree(&node.children[2]);
    
    let operator = &node.children[1];
    
    if operator.token.symbol.name == "plus" {
        
        return op1 + op2;
    }
    else {
        return op1 - op2;
    }

}

fn print_tree(node: &TreeNode)
{
    let mut level = 0;
    let mut stack: Vec<&TreeNode> = Vec::new();
    let mut next_stack: Vec<&TreeNode> = Vec::new();
    stack.push(node);

    while !stack.is_empty() {
        println!("Level {}", level);
        for item in stack {
            println!("{}", item.token.symbol.name);
            for child in &item.children {
                next_stack.push(child);
            }
        }

        stack = next_stack;
        next_stack = Vec::new();
        level += 1;
    }
}

fn main() {
    println!("Enter an expression: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to readline");
    let input = input.trim().to_string();

    let tokens = output::get_tokens(input).unwrap();

    let result = output::parse(&tokens);

    match result {
        Ok(root_node) => println!("Result: {}", recurse_tree(&root_node)),
        Err(_) => println!("Parse failed"),
    }
}