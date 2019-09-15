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
fn main() {
    println!("Hello, world!");
}
