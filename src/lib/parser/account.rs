use crate::parser::chars::LineType;
use crate::parser::{chars, Tokenizer, Directive};
use crate::{Error, ErrorType};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub(super) fn parse(tokenizer: &mut Tokenizer) -> Result<Directive, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(format!("{}{}",
        r"(account) +"        , // directive commodity
        r"(.*)"                 , // description
        ).as_str()).unwrap();
    }
    let mystr = chars::get_line(tokenizer);
    let caps = RE.captures(mystr.as_str()).unwrap();

    let mut name = String::new();
    let mut detected: bool = false;
    let mut default = false;
    let mut alias = HashSet::new();
    let mut check = Vec::new();
    let mut assert = Vec::new();
    let mut payee = Vec::new();
    let mut note = None;
    let mut isin = None;

    for (i, cap) in caps.iter().enumerate() {
        match cap {
            Some(m) => {
                match i {
                    1 =>
                    // commodity
                        {
                            detected = true;
                        }
                    2 =>
                    // description
                        {
                            name = m.as_str().to_string()
                        }
                    _ => (),
                }
            }
            None => (),
        }
    }

    if !detected {
        return Err(tokenizer.error(ErrorType::UnexpectedInput));
    }
    while let LineType::Indented = chars::consume_whitespaces_and_lines(tokenizer) {
        match tokenizer.get_char().unwrap() {
            ';' => { chars::get_line(tokenizer); }, // ignore comments
            _ => match chars::get_string(tokenizer).as_str() {
                "note" => note = Some(chars::get_line(tokenizer).trim().to_string()),
                "isin" => isin = Some(chars::get_line(tokenizer).trim().to_string()),
                "alias" => {alias.insert(chars::get_line(tokenizer).trim().to_string());},
                "check" => {check.push(chars::get_line(tokenizer).trim().to_string());},
                "assert" => {assert.push(chars::get_line(tokenizer).trim().to_string());},
                "payee" => {payee.push(chars::get_line(tokenizer).trim().to_string());},
                "default" => default = true,
                _=> {
                    eprintln!("Error while parsing posting.");
                    return Err(tokenizer.error(ErrorType::UnexpectedInput));
                }
            },
        }
    }

    Ok(Directive::Account {
        name,
        note,
        isin,
        alias,
        check,
        assert,
        payee,
        default,
    })
}
