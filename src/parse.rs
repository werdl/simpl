use crate::lex::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Node {
    pub token: Token,
    pub children: Vec<Node>,
}

pub fn parse(toks: Vec<Token>) -> Node {
    let mut nodes = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let tok = toks[i].clone();
        match tok {
            Token::If => {
                let mut children = Vec::new();
                i += 1;
                while toks[i] != Token::LeftCurlyBracket {
                    children.push(Node {
                        token: toks[i].clone(),
                        children: Vec::new(),
                    });
                    i += 1;

                    if i >= toks.len() {
                        break;
                    }
                }

                // now iterate until we find the matching right curly bracket, then append the statements as a block
                let mut block = Vec::new();
                let mut depth = 1;
                i += 1;
                if i < toks.len() {
                    while depth > 0 {
                        if toks[i] == Token::LeftCurlyBracket {
                            depth += 1;
                        } else if toks[i] == Token::RightCurlyBracket {
                            depth -= 1;
                        }
                        if depth > 0 {
                            let mut line = Vec::new();
                            while toks[i] != Token::SemiColon {
                                line.push(Node {
                                    token: toks[i].clone(),
                                    children: Vec::new(),
                                });
                                i += 1;

                                if i >= toks.len() {
                                    break;
                                }
                            }

                            line.push(
                                Node {
                                    token: Token::SemiColon,
                                    children: Vec::new(),
                                },
                            );


                            block.push(Node {
                                token: Token::Line,
                                children: line,
                            });
                        }
                        i += 1;

                        if i >= toks.len() {
                            break;
                        }
                    }
                }

                // // remove the last token, which is the right curly bracket
                // match block.pop() {
                //     Some(
                //         Node {
                //             token: Token::RightCurlyBracket,
                //             children: _,
                //         }
                //     ) => {}
                //     Some(
                //         Node {
                //             token: Token::EOF,
                //             children: _,
                //         }
                //     ) => {
                //         block.pop();
                //     }
                //     _ => {}
                // }


                children.push(Node {
                    token: Token::Block,
                    children: block,
                });

                nodes.push(Node {
                    token: Token::If,
                    children,
                });

                i -= 1;
            }
            Token::Else => {
                // else functions much like if, but without the condition
                let mut children = Vec::new();
                i += 2; // once for the else, once for the left curly bracket
                if i >= toks.len() {
                    break;
                }

                // don't worry about being prefixed by if, we sort that out later

                // now iterate until we find the matching right curly bracket, then append the statements as a block
                let mut block = Vec::new();
                let mut depth = 1;
                if i < toks.len() {
                    while depth > 0 {
                        if toks[i] == Token::LeftCurlyBracket {
                            depth += 1;
                        } else if toks[i] == Token::RightCurlyBracket {
                            depth -= 1;
                        }
                        if depth > 0 {
                            let mut line = Vec::new();
                            while toks[i] != Token::SemiColon {
                                line.push(Node {
                                    token: toks[i].clone(),
                                    children: Vec::new(),
                                });
                                i += 1;

                                if i >= toks.len() {
                                    break;
                                }
                            }
                            line.push(
                                Node {
                                    token: Token::SemiColon,
                                    children: Vec::new(),
                                },
                            );

                            block.push(Node {
                                token: Token::Line,
                                children: line,
                            });
                        }
                        i += 1;

                        if i >= toks.len() {
                            break;
                        }
                    }
                }

                children.push(Node {
                    token: Token::Block,
                    children: block,
                });

                nodes.push(Node {
                    token: Token::Else,
                    children,
                });

                i -= 1;
            }
            Token::LeftCurlyBracket => {
                let mut children = Vec::new();
                i += 1;
                while toks[i] != Token::RightCurlyBracket {
                    let mut line = Vec::new();
                    while toks[i] != Token::SemiColon {
                        line.push(Node {
                            token: toks[i].clone(),
                            children: Vec::new(),
                        });
                        i += 1;

                        if i >= toks.len() {
                            break;
                        }
                    }
                    line.push(
                        Node {
                            token: Token::SemiColon,
                            children: Vec::new(),
                        },
                    );
                    children.push(Node {
                        token: Token::Line,
                        children: line,
                    });
                    i += 1;

                    if i >= toks.len() {
                        break;
                    }
                }
                nodes.push(Node {
                    token: Token::Block,
                    children,
                });
            }
            Token::Struct => {
                /*
                struct name {
                    type name;
                    type name;
                    fn name(type name, type name) -> type {
                        ...
                    }
                }
                 */

                let mut children = Vec::new();

                // push name to children
                i += 1;

                children.push(Node { // name
                    token: toks[i].clone(),
                    children: Vec::new(),
                });

                // now iterate until we find the matching right curly bracket, gathering the tokens. then, recursively parse them

                let mut block = Vec::new();
                let mut depth = 1;
                i += 1;
                if i < toks.len() {
                    while depth > 0 {
                        if toks[i] == Token::LeftCurlyBracket {
                            depth += 1;
                        } else if toks[i] == Token::RightCurlyBracket {
                            depth -= 1;
                        }
                        if depth > 0 {
                            block.push(toks[i].clone());
                        }
                        i += 1;

                        if i >= toks.len() {
                            break;
                        }
                    }
                }

                // now remove the last token, which is the right curly bracket, and the first token, which is the left curly bracket
                match block.pop() {
                    Some(Token::RightCurlyBracket) => {}
                    Some(Token::EOF) => {
                        block.pop();
                    }
                    _ => {}
                }

                block.remove(0);

                // extract all the fields, then functions
                let mut fields = Vec::new();

                let mut j = 0;
                while j < block.len() {
                    if block[j] == Token::Fn {
                        break;
                    }
                    fields.push(Node {
                        token: block[j].clone(),
                        children: Vec::new(),
                    });
                    j += 1;
                }

                children.push(Node {
                    token: Token::Fields,
                    children: fields,
                });

                // now extract the functions
                let mut functions = Vec::new();
                while j < block.len() {
                    let mut func = Vec::new();
                    while block[j] != Token::SemiColon {
                        func.push(block[j].clone());
                        j += 1;
                    }
                    func.push(Token::SemiColon);
                    functions.push(parse(func));
                    j += 1;
                }

                children.push(Node {
                    token: Token::Functions,
                    children: functions,
                });

                nodes.push(Node {
                    token: Token::Struct,
                    children,
                });
            }
            Token::Fn => {
                // form: fn name(type arg, type arg) -> type { ... }
                let mut children = Vec::new();
                
                // push name and arguments to children
                i += 1;
                children.push(Node { // name
                    token: toks[i].clone(),
                    children: Vec::new(),
                });

                i += 2; // skip left paren
                let mut args = Vec::new();
                while toks[i] != Token::RightParen {
                    args.push(Node {
                        token: toks[i].clone(),
                        children: Vec::new(),
                    });
                    i += 1;
                }

                children.push(Node { // arguments
                    token: Token::Arguments,
                    children: args,
                });

                // skip right paren and arrow
                i += 2;

                // push return type to children
                children.push(Node { // return type
                    token: toks[i].clone(),
                    children: Vec::new(),
                });

                // now iterate until we find the matching right curly bracket, gathering the tokens. then, recursively parse them
                let mut block = Vec::new();
                let mut depth = 1;
                i += 1;
                if i < toks.len() {
                    while depth > 0 {
                        if toks[i] == Token::LeftCurlyBracket {
                            depth += 1;
                        } else if toks[i] == Token::RightCurlyBracket {
                            depth -= 1;
                        }
                        if depth > 0 {
                            block.push(toks[i].clone());
                        }
                        i += 1;

                        if i >= toks.len() {
                            break;
                        }
                    }
                }

                // now remove the last token, which is the right curly bracket, and the first token, which is the left curly bracket
                match block.pop() {
                    Some(Token::RightCurlyBracket) => {}
                    Some(Token::EOF) => {
                        block.pop();
                    }
                    _ => {}
                }


                // parse the block
                children.push(Node {
                    token: Token::Block,
                    children: vec![parse(block)],
                });


                nodes.push(Node {
                    token: Token::Fn,
                    children,
                });
            }
            Token::Identifier(_) => {
                // if it is followed by a LeftCurlyBracket, it is a struct definition

                let next = toks[i + 1].clone();

                println!("currently on {:?}", toks[i]);
                println!("next is {:?}", next);

                match next {
                    Token::LeftCurlyBracket => {
                        // handle struct definition
                        let mut children = Vec::new();

                        // push name to children
                        children.push(Node { // name
                            token: toks[i].clone(),
                            children: Vec::new(),
                        });

                        // now iterate until we find the matching right curly bracket, gathering the tokens. there will be no functions, so we don't need to parse them
                        let mut block = Vec::new();
                        let mut depth = 1;
                        i += 2;

                        if i < toks.len() {
                            while depth > 0 {
                                if toks[i] == Token::LeftCurlyBracket {
                                    depth += 1;
                                } else if toks[i] == Token::RightCurlyBracket {
                                    depth -= 1;
                                }
                                if depth > 0 {
                                    block.push(toks[i].clone());
                                }
                                i += 1;

                                if i >= toks.len() {
                                    break;
                                }
                            }
                        }

                        // now remove the last token, which is the right curly bracket, and the first token, which is the left curly bracket

                        match block.pop() {
                            Some(Token::RightCurlyBracket) => {}
                            Some(Token::EOF) => {
                                block.pop();
                            }
                            _ => {}
                        }

                        block.remove(0);

                        // extract all the fields
                        let mut fields = Vec::new();

                        let mut j = 0;

                        while j < block.len() {
                            fields.push(Node {
                                token: block[j].clone(),
                                children: Vec::new(),
                            });
                            println!("pushing {:?}", toks[i]);
                            j += 1;
                        }

                        children.push(Node {
                            token: Token::Fields,
                            children: fields,
                        });

                        nodes.push(Node {
                            token: Token::Struct,
                            children,
                        });
                    }
                    _ => {}
                }
            }
            _ => {
                // ensure to split lines by semicolons
                let mut line = Vec::new();
                while toks[i] != Token::SemiColon {
                    line.push(Node {
                        token: toks[i].clone(),
                        children: Vec::new(),
                    });
                    i += 1;

                    if i >= toks.len() {
                        break;
                    }
                }
                line.push(
                    Node {
                        token: Token::SemiColon,
                        children: Vec::new(),
                    },
                );
                nodes.push(Node {
                    token: Token::Line,
                    children: line,
                });
                
            }
        }
        i += 1;
    }
    Node {
        token: Token::Program,
        children: nodes,
    }
}
