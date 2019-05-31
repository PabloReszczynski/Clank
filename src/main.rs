use std::collections::HashMap;
use std::collections::LinkedList;

enum LispVal<'a> {
    Atom(String),
    Keyword(String),
    Number(i64),
    String(String),
    Lambda,
    Nil,
    Bool(bool),
    List(&'a LinkedList<&'a LispVal<'a>>),
}

type EnvCtx<'a> = HashMap<String, &'a LispVal<'a>>;

fn show_val(val: &LispVal) -> String {
    match val {
        LispVal::Atom(atom) => String::clone(atom),
        LispVal::Keyword(keyword) => format!(":{}", keyword),
        LispVal::String(string) => format!("\"{}\"", string),
        LispVal::Number(num) => num.to_string(),
        LispVal::Bool(true) => String::from("true"),
        LispVal::Bool(false) => String::from("false"),
        LispVal::Nil => String::from("nil"),
        LispVal::List(contents) => format!("({})", show_list(contents)),
        LispVal::Lambda => String::from("[procedure]")
    }
}

fn show_list(contents: &LinkedList<&LispVal>) -> String {
    let mut result = String::new();
    for x in contents {
        if result.is_empty() {
            result = format!("{}", show_val(x));
        } else {
            result = format!("{} {}", result, show_val(x));
        }
    }
    return result;
}

enum Token {
    OpenParen,
    CloseParen,
    Symbol(String)
}

fn lex(input: &String) -> Vec<Token> {
    let mut chars = input.chars();
    let mut tokens: Vec<Token> = Vec::new();
    for _x in 0..input.len() {
        let lookahead = chars.next();

        if let Some(character) = lookahead {
            if character == ' ' {
                continue;
            }

            let token = match character {
                '(' => Some(Token::OpenParen),
                ')' => Some(Token::CloseParen),
                _ => None
            };
        }
    }
    return tokens;
}

fn main() {
    let val1 = LispVal::Atom(String::from("hello"));
    let val2 = LispVal::Number(64);
    let val3 = LispVal::String(String::from("world"));
    let val4 = LispVal::Lambda;
    let val5 = LispVal::Nil;
    let val6 = LispVal::Bool(true);
    let val7 = LispVal::Bool(false);
    let val8 = LispVal::Bool(false);
    let val9 = LispVal::Keyword(String::from("foo"));

    let mut list: LinkedList<&LispVal> = LinkedList::new();
    list.push_front(&val1);
    list.push_front(&val2);
    list.push_front(&val3);
    list.push_front(&val4);
    list.push_front(&val5);
    list.push_front(&val6);
    list.push_front(&val7);
    list.push_front(&val8);
    list.push_front(&val9);

    let val10 = LispVal::List(&list);

    println!("{}", show_val(&val1));
    println!("{}", show_val(&val2));
    println!("{}", show_val(&val3));
    println!("{}", show_val(&val4));
    println!("{}", show_val(&val5));
    println!("{}", show_val(&val6));
    println!("{}", show_val(&val7));
    println!("{}", show_val(&val8));
    println!("{}", show_val(&val9));
    println!("{}", show_val(&val10));
}
