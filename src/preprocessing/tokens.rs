// contains the enum tokens we use

pub enum Direction {
    Left,
    Right,
}

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

pub enum Literal {
    String,
    Number,
}
    

pub enum Token {
    Whitespace,
    Brace(Direction),
    Bracket(Direction),
    Parenthesis(Direction),
    Semicolon,
    Keyword(Keyword),
    Basetype(Basetype),
    Identifier(String),
    Integer(u64),
}
