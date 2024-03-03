mod lex;
mod parse;
mod validate;

use parse::*;
use lex::*;
use validate::*;

fn main() {

    let input = "
    struct MyStruct {
        int x,
        string y
    }
    
    fn main() {
        MyStruct z = MyStruct {
            x: 0,
            y: 1   
        };
    
        print_int(z.y);

        int x = 0;
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
