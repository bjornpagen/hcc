// contains the enum tokens we use

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Keyword {
    Include,
    Define,
    Union,
    Catch,
    Class,
    Try,
    If,
    Else,
    For,
    While,
    Extern,
    Return,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Basetype {
    I0,
    I8,
    I16,
    I32,
    I64,
    U0,
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Literal {
    String,
    Number,
}
    
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Whitespace,
    Unknown,
    Brace(Direction),
    Bracket(Direction),
    Parenthesis(Direction),
    Semicolon,
    Keyword(Keyword),
    Basetype(Basetype),
    Identifier(String),
    Str(String),
    Number(u64),
}

