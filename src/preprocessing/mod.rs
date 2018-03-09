// transforms our file into (more) machine-readable tokens

pub mod tokens;

use regex::*;
use preprocessing::tokens::*;
use preprocessing::tokens::Keyword::*;
use preprocessing::tokens::Direction::*;
use preprocessing::tokens::Basetype::*;

pub fn lex(buf: &Vec<char>) -> Vec<Token> {
    let mut pos: usize = 0;
    let mut token_list: Vec<Token> = Vec::new();

    // keep generating tokens until we reach the end of the file
    let mut tup = parse_token_at(buf,pos);
    while tup.is_some() {
        let (new_tok,new_pos) = tup.unwrap();

        // ignore whitespace tokens
        if new_tok != Token::Whitespace {
            token_list.push(new_tok);
        }

        tup = parse_token_at(buf, new_pos);
    }

     token_list
}

fn parse_token_at(buf: &Vec<char>, pos: usize) -> Option<(Token, usize)> {
    // our buffer for parsing multi-char long tokens
    let mut lit_buf: Vec<char> = Vec::new();

    // check for array out of bounds
    let c = buf.get(pos);
    if c == None {
        return None
    }
    let c = c.unwrap();

    // instantly return our token if it is single char
    let tok = tokenize_single_char(*c);
    if tok.is_some() {
        let tok = tok.unwrap();
        return Some((tok, pos+1))
    } else {
        // it isn't a sigle char :(
        // time for multi-char parsing fun
        lit_buf.push(*c);
    }

    // start keeping track of char positions
    let mut local_pos = pos+1;

    // keep looking at next char (unless we are at the end of the file
    let mut current_char = buf.get(local_pos);
    while current_char.is_some()  {
        let c = current_char.unwrap();
        // if that char is single-char tokenizable, we have our final token
        if tokenize_single_char(*c).is_some() {
            return Some((tokenize_multi_char(&lit_buf[..]), local_pos));
        } else {
            // if not, do it again
            lit_buf.push(*c);
            local_pos += 1;
            current_char = buf.get(local_pos);
        }
    }
    // if we escape this while loop, it means we have reached the end of the file
    None
}

// used to parse single-char tokens
fn tokenize_single_char(c: char) -> Option<Token> {
    match c {
        '{' => Some(Token::Brace(Left)),
        '}' => Some(Token::Brace(Right)),
        '(' => Some(Token::Parenthesis(Left)),
        ')' => Some(Token::Parenthesis(Right)),
        ';' => Some(Token::Semicolon),
        ' ' => Some(Token::Whitespace),
        '\t' => Some(Token::Whitespace),
        '\r' => Some(Token::Whitespace),
        '\n' => Some(Token::Whitespace),
        _   => None,
    }
}

// converts a slice with many chars to a token
fn tokenize_multi_char(buf: &[char]) -> Token {
    lazy_static! {
         static ref RE_SET: RegexSet = RegexSet::new(&[
            r"^[IU][0,8,16,32,64]$", // basetype
            r"^[a-z]+$", // keyword
            r"^[A-z]+$", // identifier
            r"^\d+$", // number
            r#"^".*"$"#, // str
            r"^.+$", // anything else
        ]).unwrap();
    }

    let mut s = String::with_capacity(buf.len());
    for c in buf {
       s.push(c.clone());
    }

    match RE_SET.matches(s.as_str()).into_iter().nth(0) {
        Some(0) => Token::Basetype(
                match s.as_str() {
                    "I0" => Basetype::I0,
                    "I8" => Basetype::I8,
                    "I16" => Basetype::I16,
                    "I32" => Basetype::I32,
                    "I64" => Basetype::I64,
                    "U0" => Basetype::U0,
                    "U8" => Basetype::U8,
                    "U16" => Basetype::U16,
                    "U32" => Basetype::U32,
                    "U64" => Basetype::U64,
                    _ => panic!(),
                }
            ),
        Some(1) => Token::Keyword(
                match s.as_str() {
                    "include" => Keyword::Include,
                    "define" => Keyword::Define,
                    "union" => Keyword::Union,
                    "catch" => Keyword::Catch,
                    "class" => Keyword::Class,
                    "try" => Keyword::Try,
                    "if" => Keyword::If,
                    "else" => Keyword::Else,
                    "for" => Keyword::For,
                    "while" => Keyword::While,
                    "extern" => Keyword::Extern,
                    "return" => Keyword::Return,
                    _ => panic!(),
                }
            ),
        Some(2) => Token::Identifier(s),
        Some(3) => Token::Number(s.parse::<u64>().unwrap()),
        Some(4) => Token::Str({
                let len = s.len();
                let new_s = s[1..len-1].to_string();
                new_s
            }),
        Some(5) => Token::Unknown,
        _ => panic!(),
    }
}

