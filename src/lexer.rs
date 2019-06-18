#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Token {
  BracketOpen,        // [
  BracketClose,       // ]
  ParensOpen,         // (
  ParensClose,        // )
  CurlyOpen,          // {
  CurlyClose,         // }
  Quote,              // '
  Integer(i32),       // 5
  Float(f32),         // 1.3
  Fraction(i32, i32), // 2/5
  Str(String),        // "foo"
  Keyword(String),    // :foo
  Symbol(String),     // foo
}

fn lex(source: String) -> Vec<Token> {
  let new_source = source
    .replace("(", " ( ")
    .replace(")", " ) ")
    .replace("[", " [ ")
    .replace("]", " ] ")
    .replace("{", " { ")
    .replace("}", " } ")
    .replace("'", " ' ");

  let splitted_source = new_source.split_whitespace();
  splitted_source
    .map(|tok| match tok {
      "(" => Token::ParensOpen,
      ")" => Token::ParensClose,
      "[" => Token::BracketOpen,
      "]" => Token::BracketClose,
      "}" => Token::CurlyOpen,
      "{" => Token::CurlyClose,
      "'" => Token::Quote,
      x => {
        if let Ok(i) = x.parse::<i32>() {
          Token::Integer(i)
        } else if let Ok(i) = x.parse::<f32>() {
          Token::Float(i)
        } else {
          // fraction
          let values: Vec<&str> = x.split('/').collect();
          if values.len() == 2 {
            let first: &str = values.get(0).unwrap();
            let secnd: &str = values.get(1).unwrap();
            if let Ok(num) = first.parse::<i32>() {
              if let Ok(den) = secnd.parse::<i32>() {
                return Token::Fraction(num, den);
              }
            }
          }
          // keyword
          if let Some(colon) = x.chars().nth(0) {
            if colon == ':' {
              return Token::Keyword(x.get(1..).unwrap().to_string());
            }
          }
          return Token::Symbol(x.to_string());
        }
      }
    })
    .collect()
}

#[cfg(test)]
mod lexer_tests {
  use super::*;

  #[test]
  fn test_open_parens() {
    let s = String::from("(");
    let expected = vec![Token::ParensOpen];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_symbol() {
    let s = String::from("hello");
    let expected = vec![Token::Symbol(String::from("hello"))];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_integer() {
    let s = String::from("19");
    let expected = vec![Token::Integer(19)];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_float() {
    let s = String::from("25.564");
    let expected = vec![Token::Float(25.564)];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_fraction() {
    let s = String::from("4/5");
    let expected = vec![Token::Fraction(4, 5)];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_list() {
    let s = String::from("(+ 10 20.0 30/5)");
    let expected = vec![
      Token::ParensOpen,
      Token::Symbol(String::from("+")),
      Token::Integer(10),
      Token::Float(20.0),
      Token::Fraction(30, 5),
      Token::ParensClose,
    ];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_keyword() {
    let s = String::from(":foo");
    let expected = vec![Token::Keyword(String::from("foo"))];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_str() {
    let s = String::from("\"hello, world\"");
    let expected = vec![Token::Str(String::from("hello, world"))];
    let result = lex(s);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_vector() {}
}
