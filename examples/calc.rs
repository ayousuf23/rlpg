mod result;
use result::TreeNode;

fn get_true_size(nodes: &Vec<TreeNode>) -> usize
{
    for node in nodes {
        if node.token.symbol.name == "eof" {
            return nodes.len() - 1;
        }
    }
    return nodes.len();
}

fn recurse_tree(node: &TreeNode) -> i32
{
    if node.token.symbol.name == "term" {
        // Check num children
        if get_true_size(&node.children) == 1 {
            return recurse_tree(&node.children[0]);
        }
        else {
            println!("{:?}", node);
            let lhs = recurse_tree(&node.children[0]);
            println!("lhs: {}", lhs);
            let rhs = recurse_tree(&node.children[2]);
            if node.children[1].token.symbol.name == "times" {
                return lhs * rhs;
            }
            else {
                return lhs / rhs;
            }
        }
    }
    else if node.token.symbol.name == "number" {
        return node.token.lexeme.parse::<i32>().unwrap();
    }
    else if node.token.symbol.name == "expression" {
        // Check num children
        if get_true_size(&node.children) == 1 {
            return recurse_tree(&node.children[0]);
        }
        else {
            let lhs = recurse_tree(&node.children[0]);
            let rhs = recurse_tree(&node.children[2]);
            if node.children[1].token.symbol.name == "plus" {
                return lhs + rhs;
            }
            else {
                return lhs - rhs;
            }
        }
    }
    else {
        return recurse_tree(&node.children[0]);
    }
}

fn main() {
    println!("Enter an expression: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to readline");
    let input = input.trim().to_string();

    let tokens = result::get_tokens(input).unwrap();

    let t = result::parse(&tokens);

    println!("Result: {}", recurse_tree(&t.unwrap()));
}