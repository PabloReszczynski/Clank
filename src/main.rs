//use std::collections::HashMap;
use std::collections::LinkedList;
mod reader;

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

//type EnvCtx<'a> = HashMap<String, &'a LispVal<'a>>;

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

#[test]
fn test() {
    let val1 = LispVal::Atom(String::from("hello"));
    let val2 = LispVal::Number(64);
    let val3 = LispVal::String(String::from("world"));
    let val4 = LispVal::Lambda;
    let val5 = LispVal::Nil;
    let val6 = LispVal::Bool(true);
    let val7 = LispVal::Bool(false);
    let val8 = LispVal::Keyword(String::from("foo"));

    let mut list: LinkedList<&LispVal> = LinkedList::new();
    list.push_front(&val1);
    list.push_front(&val2);
    list.push_front(&val3);
    list.push_front(&val4);
    list.push_front(&val5);
    list.push_front(&val6);
    list.push_front(&val7);
    list.push_front(&val8);

    let val9 = LispVal::List(&list);

    assert!(show_val(&val1) == String::from("hello"));
    assert!(show_val(&val2) == String::from("64"));
    assert!(show_val(&val3) == String::from("\"world\""));
    assert!(show_val(&val4) == String::from("[procedure]"));
    assert!(show_val(&val5) == String::from("nil"));
    assert!(show_val(&val6) == String::from("true"));
    assert!(show_val(&val7) == String::from("false"));
    assert!(show_val(&val8) == String::from(":foo"));
    assert!(show_val(&val9) == String::from("(:foo false true nil [procedure] \"world\" 64 hello)"));
}

fn main() {}
