use std::collections::LinkedList;

pub struct AST {}

pub fn parse(src: String) -> AST {
    AST{}
}

fn tokenizer(src: String) -> Vec<String> {
    let mut tokens = Vec::<String>::new();
    let mut chars = src.chars().peekable();
    tokens
}

fn is_balanced(src: String) -> bool {
    let mut stack = LinkedList::<char>::new();
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => stack.push_front(c),
            ')' => {
                let paren = stack.pop_front();
                match paren {
                    Some('(') => continue,
                    _ => return false,
                }
            },
            _ => continue
        }
    }
    if stack.is_empty() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod parser_tests {

    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = String::from("(+ 1 2)");
        let t1 = String::from("+");
        let t2 = String::from("1");
        let t3 = String::from("2");
        let t4 = String::from("(");
        let t5 = String::from(")");

        assert_eq!(tokenizer(src), vec![t4, t1, t2, t3, t5]);
    }

    #[test]
    fn test_is_balanced() {
        let str1 = String::from("()");
        let str2 = String::from("(");
        let str3 = String::from(")");
        let str4 = String::from("(())");
        let str5 = String::from("())");
        assert!(is_balanced(str1));
        assert!(!is_balanced(str2));
        assert!(!is_balanced(str3));
        assert!(is_balanced(str4));
        assert!(!is_balanced(str5));
    }
}

