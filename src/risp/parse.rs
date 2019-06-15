/******************************************************************************
** @crates and modules
******************************************************************************/

extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::sync::Arc;

use crate::risp::{RErr, RStr, RSym, RVal, RVal::*};

/******************************************************************************
** @lexer
******************************************************************************/

pub fn tokenize<S>(src: S) -> Vec<String>
where
    S: Into<String>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?mx)
        [\s]*(,@|[\[\]{}()'`,^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)
        "#
        )
        .unwrap();
    }
    let mut tokens = Vec::new();
    for capture in RE.captures_iter(&src.into()) {
        if !capture[1].starts_with(';') {
            tokens.push(capture[1].to_string());
        }
    }
    tokens
}

/******************************************************************************
** @parser
******************************************************************************/

pub fn parse<'a>(tokens: &'a [String]) -> Result<(RVal, &'a [String]), RVal> {
    let (head, rest) = tokens.split_first().ok_or_else(|| RErrUnexpected!("EOF"))?;
    match &head[..] {
        "(" => read_rest(rest, ")"),
        ")" => Err(RErrUnexpected!(")")),
        "[" => read_rest(rest, "]"),
        "]" => Err(RErrUnexpected!("]")),
        "{" => read_rest(rest, "}"),
        "}" => Err(RErrUnexpected!("}")),
        _ => {
            let atom = parse_atom(head);
            match atom {
                _RErr(_) => Err(atom),
                _ => Ok((atom, rest)),
            }
        }
    }
}

// TODO: add support for hashmaps and sets
fn read_rest<'a>(tokens: &'a [String], end: &str) -> Result<(RVal, &'a [String]), RVal> {
    let mut vs = vec![];
    let mut xs = tokens;
    loop {
        let (next, rest) = xs.split_first().ok_or_else(|| RErrExpected!(end, "EOF"))?;
        if next == end {
            return Ok((RVec(Arc::new(vs)), rest)); // TODO: macro that inserts nil at end
        }
        let (new_vs, new_xs) = parse(&xs)?;
        vs.push(new_vs);
        xs = new_xs;
    }
}

fn parse_atom(atom: &str) -> RVal {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(
            r#"(?mx)
            (^[-+]?([1-9]\d*|0)$)"#
        )
        .unwrap();
        static ref FLT_RE: Regex = Regex::new(
            r#"(?mx)
            (^[-+]?(\.[0-9]+|(([1-9]\d*|0)\.[0-9]*))$)"#
        )
        .unwrap();
        static ref STR_RE: Regex = Regex::new(
            r#"(?mx)
            "(?:\\.|[^\\"])*""#
        )
        .unwrap();
    }
    match atom {
        "nil" => RNil,
        "false" => RBool(false),
        "true" => RBool(true),
        _ => {
            if INT_RE.is_match(&atom) {
                let num = atom.parse();
                match num {
                    Ok(i) => RInt(i),
                    Err(_) => RErr("integer overflow"),
                }
            } else if FLT_RE.is_match(&atom) {
                let num = atom.parse();
                match num {
                    Ok(f) => RFlt(f),
                    Err(_) => RErr("floating point overflow"),
                }
            } else if STR_RE.is_match(&atom) {
                RStr(unescape(&atom[1..atom.len() - 1]))
            } else if atom.starts_with('"') {
                RErrExpected!("\"", "EOF")
            } else {
                RSym(atom)
            }
        }
    }
}

fn unescape<S>(src: S) -> String
where
    S: Into<String>,
{
    src.into()
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\\\", "\\")
}
