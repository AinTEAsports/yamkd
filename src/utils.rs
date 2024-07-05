use crate::stack::Stack;

const INVALID_CHARS_COUNT: usize = 1;
const INVALID_CHARS: [char; INVALID_CHARS_COUNT] = ['/'];

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


pub fn is_valid_expression(s: &str) -> bool {
    is_valid_parenthesis(s) && !contains_any(s, INVALID_CHARS)
}
