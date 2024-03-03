mod lex;
mod parse;
mod validate;

use parse::*;
use lex::*;
use validate::*;

fn main() {

    let input = "
fn main() -> int {
    if (1 == 1) {
        return(1);
    }

return(0);
print(1);
}
";
    let mut lexer = Lexer::new(input.to_string());

    let mut tokens = Vec::new();

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        tokens.push(tok.clone());

        if tok == Token::EOF {
            break;
        }
    }

    let ast = parse(tokens);

    println!("{:#?}", ast);

    println!("{}", validate(ast));
}
