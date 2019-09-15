use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum TokenType {
    Comma,
    Number,
    Identifier,
    Whitespace, Newline, EOF
}

#[derive(Debug)]
struct Token {
    kind: Option<TokenType>,
    data: Option<String>,
    start: Option<usize>,
    position: Option<(usize, usize)>
}

fn tokenize(code: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut last_token_start: Option<usize> = None;
    let mut last_kind: Option<TokenType> = None;

    for (i, c) in code.chars().enumerate() {
        let kind = match c {
            ',' => Some(TokenType::Comma),
            '\t' | '\r' | ' ' => Some(TokenType::Whitespace),
            '\0' => Some(TokenType::EOF),
            '\n' => Some(TokenType::Newline),

            // Identifiers adhere to the following regex: ([a-z]|[A-Z]|\_)+
            'A'..='Z' | 'a'..='z' => Some(TokenType::Identifier),
            '_' => if let Some(TokenType::Identifier) = last_kind { Some(TokenType::Identifier) } else { None },

            // Numbers adhere to the following regex: ([0-9]|\.)+
            '0'..='9' => Some(TokenType::Number),
            '.' => if let Some(TokenType::Number) = last_kind { Some(TokenType::Number) } else { None },

            // Catchall trait
            _ => None
        };

        if let Some(prev) = &last_kind {
            if let Some(curr) = &kind {
                if prev != curr { // End of last token; Start of new token
                    println!("Tokenchange; {:?}, {:?}, {}", prev, curr, i);
                    let token = match last_token_start {
                        Some(s) => {
                            Token {
                                kind: last_kind,
                                data: Some(String::from(&code[s .. i])),
                                start: Some(s),
                                position: Some((0, 0)) // TODO - Implement
                            }
                        },
                        None => {
                            Token {
                                kind: None,
                                data: None,
                                start: None,
                                position: None
                            }
                        }
                    };
                    tokens.push(token);

                    last_token_start = Some(i);
                }
            }
        } else { // First token - no last_kind
            last_token_start = Some(0);
        }

        last_kind = kind;
    }

    tokens 
}

fn main() {
    let line = String::from("read_comp A, 5.1, 6.2\n\nprint_comp A\0");
    let tokens = tokenize(&line);
    println!("{:?}", tokens);
}
