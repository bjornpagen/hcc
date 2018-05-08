// transforms our file into (more) machine-readable tokens

use regex::*;
use token::*;
use token::Direction::*;
use token::Operator::*;

pub fn lex(buf: &Vec<char>) -> Vec<Token> {
    let pos: usize = 0;
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
    if buf.get(pos).is_none() {
        return None
    }

    // instantly return our token if it is single char
    let tup = tokenize_single_char(&buf,pos);
    if tup.is_some() {
        return tup;
    } else {
        // it isn't a sigle char :(
        // time for multi-char parsing fun
        lit_buf.push(buf[pos]);
    }

    // start keeping track of char positions
    let mut local_pos = pos+1;

    // keep looking at next char (unless we are at the end of the file
    let mut current_char = buf.get(local_pos);
    while current_char.is_some()  {
        // if that char is single-char tokenizable, we have our final token
        if tokenize_single_char(&buf,local_pos).is_some() {
            return Some((tokenize_multi_char(&lit_buf[..]), local_pos));
        } else {
            // if not, do it again
            lit_buf.push(buf[local_pos]);
            local_pos += 1;
            current_char = buf.get(local_pos);
        }
    }
    // if we escape this while loop, it means we have reached the end of the file
    None
}

// used to parse single-char tokens
fn tokenize_single_char(buf: &[char], pos: usize) -> Option<(Token,usize)> {
    let sin_tok = |tok| Some((tok,pos+1));

    // matching here is constant time, since hopefully LLVM jump tables this
    match buf[pos] {
        ' ' => sin_tok(Token::Whitespace),
        '{' => sin_tok(Token::Brace(Left)),
        '}' => sin_tok(Token::Brace(Right)),
        '(' => sin_tok(Token::Parenthesis(Left)),
        ')' => sin_tok(Token::Parenthesis(Right)),
        ';' => sin_tok(Token::Semicolon),
        '+' => sin_tok(Token::Operator(Add)),
        '-' => sin_tok(Token::Operator(Sub)),
        '*' => sin_tok(Token::Operator(Star)),
        '/' => {
            // this is ugly and awful comment detection
            // comments are stupid
            let c = buf.get(pos+1);
            if c.is_some() && *c.unwrap() == '/' {
                let mut local_pos = pos+2;
                let mut test_c = buf.get(local_pos);
                while test_c.is_some() && *test_c.unwrap() != '\n' {
                    local_pos+=1;
                    test_c = buf.get(local_pos);
                }
                return Some((Token::Whitespace,local_pos+1));
            } else {
                return sin_tok(Token::Operator(Div));
            }
        },
        '=' => sin_tok(Token::Operator(Equal)),
        '>' => sin_tok(Token::Operator(Greater)),
        '<' => sin_tok(Token::Operator(Less)),
        ',' => sin_tok(Token::Comma),
        '"' => {
            // quotations are for strings
            let tup = tokenize_str(&buf,pos);
            if tup.is_some() {
                return tup;
            } else {
                panic!("string expected, closing quotation not found...");
            }
        },
        '\t' => sin_tok(Token::Whitespace),
        '\r' => sin_tok(Token::Whitespace),
        '\n' => sin_tok(Token::Whitespace),
        _   => None,
    }
}

// converts a slice with many chars to a token
fn tokenize_multi_char(buf: &[char]) -> Token {
    lazy_static! {
         static ref RE_SET: RegexSet = RegexSet::new(&[
            r"^[IU](0|8|16|32|64)$", // basetype
            r"^\d+$", // number
            r"^[\w]+$", // anything with letters only
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
        Some(1) => Token::Number(s.parse::<u64>().unwrap()),
        Some(2) => {
                match s.as_str() {
                    "include" => Token::Keyword(Keyword::Include),
                    "define" => Token::Keyword(Keyword::Define),
                    "union" => Token::Keyword(Keyword::Union),
                    "catch" => Token::Keyword(Keyword::Catch),
                    "class" => Token::Keyword(Keyword::Class),
                    "try" => Token::Keyword(Keyword::Try),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "for" => Token::Keyword(Keyword::For),
                    "while" => Token::Keyword(Keyword::While),
                    "extern" => Token::Keyword(Keyword::Extern),
                    "return" => Token::Keyword(Keyword::Return),
                    _ => Token::Identifier(s),
                }
            },
        Some(3) => Token::Unknown,
        _ => panic!(),
    }
}

fn tokenize_str(buf: &[char], pos: usize) -> Option<(Token,usize)> {
    let our_char = *buf.get(pos).unwrap();
    let mut str_buf = String::new();
    let mut local_pos = pos+1;

    let mut c = buf.get(local_pos);
    while c.is_some() {
        if *c.unwrap() == our_char {
            return Some((Token::Str(str_buf),local_pos+1));
        }
        str_buf.push(*c.unwrap());
        local_pos+=1;
        c = buf.get(local_pos);
    }

    // if we escape this loop, we have reached EOF
    return None
}

