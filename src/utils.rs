use crate::stack::Stack;
use crate::parser::{ get, INNER_SEPARATOR, OUTER_SEPARATOR };

const INVALID_CHARS_COUNT: usize = 1;
const INVALID_CHARS: [char; INVALID_CHARS_COUNT] = ['/'];

pub fn error_str(msg: &str) -> String {
    let bold_grey = "\x1b[1;30m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    format!("{bold_grey}[{reset}{red}ERROR{reset}{bold_grey}]{reset} {msg}")
}

fn is_valid_parenthesis(s: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();

    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => match stack.pop() {
                Some('(') => (),
                _ => return false,
            }
            _ => ()
        }
    }

    stack.is_empty()
}


fn contains_any(s: &str, chars: [char; INVALID_CHARS_COUNT]) -> bool {
    for c in s.chars() {
        for c2 in chars {
            if c == c2 { return true; }
        }
    }

    false
}


fn contains_two_consecutive(s: &str, c: char) -> bool {
    for i in 0..s.len()-1 {
        if get(s, i) == Some(c) && get(s, i+1) == Some(c) { return true; }
    }

    false
}


pub fn find_indexes(s: &str, separator: char) -> Option<Vec<usize>> {
    let mut stack: Stack<char> = Stack::new();
    let mut result = vec![];

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => stack.push(c),
            ')' => match stack.pop() {
                Some('(') => (),
                _ => return None,
            }
            c if c == separator && stack.is_empty() => result.push(i),
            _ => ()
        }
    }

    if stack.is_empty() { Some(result) }
    else { None }
}


pub fn split_at(s: String, positions: Vec<usize>) -> Vec<String> {
    let mut result = vec![];
    let mut rest = s;

    let mut n = 0;

    for pos in positions {
        let (fst, snd) = rest.split_at(pos-n);

        n += fst.len()+1;
        result.push(format!("{}{}", fst, OUTER_SEPARATOR));
        rest = snd[1..].to_string();
    }

    if !rest.is_empty() { result.push(rest); }

    result
}


pub fn is_valid_expression(s: &str) -> Result<(), &str> {
    if s.is_empty() { Err("empty expression") }
    else if !is_valid_parenthesis(s) { Err("invalid parenthesis") }
    else if s.chars().next().unwrap() == OUTER_SEPARATOR { Err("first character cannot be '/'") }
    else if s.contains("//") { Err("cannot contain two consecutive '/'") }
    else if s.contains(",,") { Err("cannot contain two consecutive ','") }
    else { Ok(()) }
}


pub fn get_lastindex(s: &str, c: char) -> Option<usize> {
    for i in (0..s.len()).rev() {
        if s.chars().nth(i).unwrap() == c { return Some(i); }
    }

    None
}
