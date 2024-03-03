use crate::parse::*;
use crate::Token;

pub fn validate(node: Node) -> bool {
    match node.token {
        Token::If => {
            if node.children.len() < 2 {
                return false;
            }
            if !validate(node.children[0].clone()) {
                return false;
            }
            if !validate(node.children[1].clone()) {
                return false;
            }
        }
        Token::Block => {
            for child in node.children {
                if !validate(child) {
                    return false;
                }
            }
        }
        Token::Number(_) => {}
        Token::Identifier(_) => {}
        _ => {
            return false;
        }
    }
    true
}