#![allow(dead_code)]

use std::collections::LinkedList;

pub struct AST {}

struct Paren<'a> {
  pub ch: &'a str,
}

pub fn parse(_src: String) -> AST {
  AST {}
}

// fn tokenize<'a>(src: String) -> Vec<&'a str> {
//     let s = src.replace("(", " ( ").replace(")", " ) ");
//     let split = s.split_whitespace();
//     let coll = split.collect::<Vec<&'a str>>();
//     s
//     coll
// }

fn match_paren(paren: &str) -> Option<&'static str> {
  match paren {
    "{" => Some("}"),
    "}" => Some("{"),
    "(" => Some(")"),
    ")" => Some("("),
    "[" => Some("]"),
    "]" => Some("["),
    _ => None,
  }
}

fn is_valid_close_paren<'a>(paren_stack: &Vec<Paren<'a>>, ch: &'a str) -> bool {
  if paren_stack.is_empty() {
    return false;
  }
  if let Some(paren) = peek(paren_stack, 0) {
    if let Some(close) = match_paren(ch) {
      if paren.ch == close {
        return true;
      }
    }
  }
  false
}

fn peek<T>(array: &Vec<T>, i: usize) -> Option<&T> {
  if i >= array.len() {
    None
  } else {
    Some(&array[array.len() - 1 - i])
  }
}

fn is_balanced<'a>(src: &'a str) -> bool {
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
      }
      _ => continue,
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
  fn test_tokenize() {
    let src = String::from("(+ 1 2)");
    let t1 = String::from("+");
    let t2 = String::from("1");
    let t3 = String::from("2");
    let t4 = String::from("(");
    let t5 = String::from(")");

    //assert_eq!(tokenize(src), vec![t4, t1, t2, t3, t5]);
  }

  #[test]
  fn test_is_balanced() {
    let str1 = "()";
    let str2 = "(";
    let str3 = ")";
    let str4 = "(())";
    let str5 = "())";
    assert!(is_balanced(str1));
    assert!(!is_balanced(str2));
    assert!(!is_balanced(str3));
    assert!(is_balanced(str4));
    assert!(!is_balanced(str5));
  }
}
