// transforms our file bufferlinto (more) machine-readable tokens

mod tokens;

pub fn lex(buf: &Vec<char>) -> Vec<Token> {
    let mut pos: usize = 0;
    let mut token_list: Vec<Token> = Vec::new();

    // keep generating tokens until we reach the end of the file
    let mut tup = parse_token_at(buf,pos);
    while tup != None {
        Some(new_tok,new_pos) = tup;

        // ignore whitespace tokens
        if new_tok != Token::Whitespace {
            token_list.push(new_tok);
        }

        tup = parse_token_at(buf, new_pos);
    }

     token_list
}

fn parse_token_at(buf: &Vec<char>, pos: usize) -> Option(Token, usize) {
    // closure to recognize tokenizable single chars
    let tokenize_single_char = |tok| {
        match c {
            '{' => Token::Brace::Left,
            '}' => Token::Brace::Right,
            '(' => Token::Parenthesis::Left,
            ')' => Token::Parenthesis::Right,
            ';' => Token::Semicolon,
            ' ' => Token::Whitespace,
            _   => None,
        }
    }

    // our buffer for parsing multi-char long tokens
    let mut lit_buf: Vec<char> = Vec::new();

    // instantly return our token if it is single char
    let tok = tokenize_single_char(buf[pos]);
    if tok != None {
        let Some(tok) = tok;
        return Some(tok, pos+1)
    } else {
        // it isn't a sigle char :(
        // time for multi-char parsing fun
        lit_buf.push(buf[pos]);
    }

    // start keeping track of char positions
    let mut local_pos = pos+1;

    // keep looking at next char (unless we are at the end of the file
    let mut c = buf.get(local_pos);
    while c != None {
        let Some(c) = c;
        // if that char is single-char tokenizable, we have our final token
        if tokenize_single_char(c) != None {
            return Some(tokenize_multi_char(&lit_buf[..]), local_pos);
        } else {
            // if not, do it again
            lit_buf.push(c);
            local_pos += 1;
        }
    }
    // if we escape this while loop, it means we have reached the end of the file
    None
}

// converts a slice with many chars to a token
fn tokenize_multi_char(&[char]) -> Token {

}

