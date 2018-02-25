// contains the enum tokens we use

enum Brace {
    Left,
    Right,
}

enum Parenthesis {
    Left,
    Right,
}

enum Keyword {
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

enum Basetype {
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

enum Literal {
    String,
    Number,
}
    

pub enum Token {
    Whitespace, // whitespace tokens should be eliminated in the parse phase
    Brace,
    Parenthesis,
    Semicolon,
    Keyword,
    Basetype,
    Identifier(String),
    Integer(u64),
}
