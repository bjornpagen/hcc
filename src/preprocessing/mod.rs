// transforms our file into (more) machine-readable tokens

pub mod tokens;

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
        let Some((new_tok,new_pos)) = tup;

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

    // instantly return our token if it is single char
    let tok = tokenize_single_char(buf[pos]);
    if tok.is_some() {
        let Some(tok) = tok;
        return Some((tok, pos+1))
    } else {
        // it isn't a sigle char :(
        // time for multi-char parsing fun
        lit_buf.push(buf[pos]);
    }

    // start keeping track of char positions
    let mut local_pos = pos+1;

    // keep looking at next char (unless we are at the end of the file
    let mut c = buf.get(local_pos);
    while c.is_some()  {
        let Some(c) = c;
        // if that char is single-char tokenizable, we have our final token
        if tokenize_single_char(*c).is_some() {
            return Some((tokenize_multi_char(&lit_buf[..]), local_pos));
        } else {
            // if not, do it again
            lit_buf.push(*c);
            local_pos += 1;
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

}

