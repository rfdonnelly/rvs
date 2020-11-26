use std::str::FromStr;
use std::char;
use std::path::{Path, MAIN_SEPARATOR};
use std::fs::File;
use std::io::prelude::*;

use crate::sourcepaths::SourcePaths;

use crate::ast::{
    VariableMethod,
    BinaryOpcode,
    UnaryOpcode,
    Type,
    Node,
    Item,
    Replacement,
};

pub use grammar::*;

peg::parser!{grammar grammar() for str {
    rule import_path() -> &'input str
        = quiet!{$([':' | 'a'..='z' | 'A'..='Z' | '_']+)} / expected!("import path")

    rule import(import_paths: &mut SourcePaths) -> Item
        = "import" _ s:import_path() _ ";" {
            let path = Path::new(&s.replace("::", &MAIN_SEPARATOR.to_string())).with_extension("rvs");
            match import_paths.find(&path) {
                Err(e) => {
                    Item::ImportError(path.to_path_buf(), e)
                }
                Ok(path) => {
                    if import_paths.enter_import(&path) {
                        match File::open(&path) {
                            Err(e) => {
                                Item::ImportError(path.to_path_buf(), e)
                            }
                            Ok(mut file) => {
                                let mut contents = String::new();
                                match file.read_to_string(&mut contents) {
                                    Err(e) => {
                                        Item::ImportError(path.to_path_buf(), e)
                                    }
                                    Ok(_) => {
                                        let result = Item::Multiple(items(&contents, import_paths).unwrap());
                                        import_paths.leave_import();

                                        result
                                    }
                                }
                            }
                        }
                    } else {
                        Item::Multiple(Vec::new())
                    }
                }
            }
        }

    rule identifier() -> &'input str
        = quiet!{$(['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | ':']*)} / expected!("variable name")

    rule type_name() -> &'input str
        = quiet!{$(['A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*)} / expected!("type name")

    rule dec_digit() = ['0'..='9']
    rule dec_number() -> u32
        = s:$(dec_digit() (dec_digit() / "_")*) {
            let stripped = &str::replace(s, "_", "");
            u32::from_str(stripped).unwrap()
        }

    rule hex_digit() = ['0'..='9' | 'a'..='f' | 'A'..='F']
    rule hex_number() -> u32
        = "0" ['x' | 'X'] s:$(hex_digit() (hex_digit() / "_")*) {
            let stripped = &str::replace(s, "_", "");
            u32::from_str_radix(stripped, 16).unwrap()
        }

    rule number() -> Box<Node>
        = u:hex_number() { Box::new(Node::Number(u)) }
        / u:dec_number() { Box::new(Node::Number(u)) }

    rule r_identifier() -> Box<Node>
        = a:identifier() b:variable_method_call()? {
            let method = if let Some(method) = b {
                method
            } else {
                VariableMethod::Next
            };

            Box::new(Node::RIdentifier(a.to_owned(), method))
        }

    rule variable_method_call() -> VariableMethod
        = "." a:variable_method() { a }

    rule variable_method() -> VariableMethod
        = "prev" { VariableMethod::Prev }
        / "copy" { VariableMethod::Copy }

    rule typ() -> Box<Node>
        = pattern()
        / range()
        / weighted()
        / sequence()
        / done()
        / once()

    rule expr() -> Box<Node> = precedence!{
        x:(@) _ "|" _  y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Or, y)) }
        --
        x:(@) _ "^" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Xor, y)) }
        --
        x:(@) _ "&" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::And, y)) }
        --
        x:(@) _ "<<" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Shl, y)) }
        x:(@) _ ">>" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Shr, y)) }
        --
        x:(@) _ "+" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Add, y)) }
        x:(@) _ "-" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Sub, y)) }
        --
        x:(@) _ "*" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Mul, y)) }
        x:(@) _ "/" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Div, y)) }
        x:(@) _ "%" _ y:@ { Box::new(Node::BinaryOperation(x, BinaryOpcode::Mod, y)) }
        --
        "(" _ v:expr() _ ")" { v }
        "~" _ v:@ { Box::new(Node::UnaryOperation(UnaryOpcode::Inv, v)) }
        "-" _ v:@ { Box::new(Node::UnaryOperation(UnaryOpcode::Neg, v)) }
        v:number() { v }
        v:typ() { v }
        v:r_identifier() { v }
    }

    rule item(import_paths: &mut SourcePaths) -> Item
        = enum()
        / variable()
        / import(import_paths)

    rule variable() -> Item
        = lhs:identifier() _ "=" _ rhs:expr() _ ";" {
            Item::Single(
                Box::new(Node::Variable(lhs.into(), rhs))
            )
        }

    pub rule items(import_paths: &mut SourcePaths) -> Vec<Item>
        = _ a:item(import_paths) ** _ _ { a }

    rule optional_trailing_comma()
        = (_ "," _)?

    rule enum() -> Item
        = "enum" _ id:type_name() _ "{" _ enum_members:enum_member() ** ("," _) optional_trailing_comma() _ "}" {
            Item::Single(
                Box::new(Node::Enum(id.into(), enum_members))
            )
        }

    rule enum_assignment() -> Box<Node>
        = "=" _ a:number() _ { a }

    rule enum_member() -> Box<Node>
        = a:type_name() _ b:enum_assignment()? {
            Box::new(Node::EnumMember(a.into(), b))
        }

    rule pattern() -> Box<Node>
        = "Pattern" _ "(" _ a:expr() ++ ("," _) optional_trailing_comma() _ ")" {
            Box::new(Node::Type(Type::Pattern, a))
        }

    rule sequence() -> Box<Node>
        = "Sequence" _ "(" _ a:expr() **<1, 3> ("," _) optional_trailing_comma() _ ")" {
            Box::new(Node::Type(Type::Sequence, a))
        }

    rule expand() -> Box<Node>
        = "Expand" _ "(" _ a:expr() **<1, 2> ("," _) optional_trailing_comma() _ ")" {
            Box::new(Node::Type(Type::Expand, a))
        }

    rule range() -> Box<Node>
        = "[" _ a:expr() **<2> ("," _) _ "]" {
            Box::new(Node::Type(Type::Range, a))
        }

    rule weighted_sample() -> Box<Node>
        = a:weight()? b:expr() {
            let weight = match a {
                Some(weight) => weight,
                None => 1,
            };
            Box::new(Node::WeightedSample(weight, b))
        }

    rule weight() -> u32
        = a:dec_number() _ ":" _ { a }

    rule weighted() -> Box<Node>
        = replacement:"r"?"{" _ entries:(weighted_sample() / expand()) ++ ("," _) optional_trailing_comma() _ "}" {
            let replacement = match replacement {
                Some(_) => Replacement::With,
                None => Replacement::Without,
            };
            Box::new(Node::Weighted(replacement, entries))
        }

    rule done() -> Box<Node>
        = "Done" _ "(" _ a:expr() _ ")" {
            Box::new(Node::Type(Type::Done, vec![a]))
        }

    rule once() -> Box<Node>
        = "Once" _ "(" _ a:expr() _ ")" {
            Box::new(Node::Type(Type::Once, vec![a]))
        }

    // From: https://github.com/kevinmehall/rust-peg/blob/cc6a3cdebfafc670a9dffb0422709ff6d85d1207/src/grammar.rustpeg
    rule _() = quiet!{(whitespace() / eol() / comment())*}

    rule comment() = singleLineComment()

    rule singleLineComment()
        = "//" (!eolChar() [_])*

    // Modeled after ECMA-262, 5th ed., 7.3.
    rule eol()
        = "\n"
        / "\r\n"
        / "\r"
        / "\u{2028}"
        / "\u{2029}"

    rule eolChar()
      = ['\n' | '\r' | '\u{2028}' | '\u{2029}']

    // Modeled after ECMA-262, 5th ed., 7.2.
    rule whitespace()
        = [' ' | '\t' | '\u{00A0}' | '\u{FEFF}' | '\u{1680}' | '\u{180E}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'] // \v\f removed

    rule string() -> String
        = string:(doubleQuotedString() / singleQuotedString()) { string }

    rule doubleQuotedString() -> String
        = "\"" s:doubleQuotedCharacter()* "\"" { s.into_iter().collect() }

    rule doubleQuotedCharacter() -> char
        = simpleDoubleQuotedCharacter()
        / simpleEscapeSequence()
        / zeroEscapeSequence()
        / hex2EscapeSequence()
        / unicodeEscapeSequence()
        / eolEscapeSequence()

    rule simpleDoubleQuotedCharacter() -> char
        = !("\"" / "\\" / eolChar()) c:$[_] { c.chars().next().unwrap() }

    rule singleQuotedString() -> String
        = "'" s:singleQuotedCharacter()* "'" { s.into_iter().collect() }

    rule singleQuotedCharacter() -> char
        = simpleSingleQuotedCharacter()
        / simpleEscapeSequence()
        / zeroEscapeSequence()
        / hex2EscapeSequence()
        / unicodeEscapeSequence()
        / eolEscapeSequence()

    rule simpleSingleQuotedCharacter() -> char
        = !("'" / "\\" / eolChar()) c:$[_] { c.chars().next().unwrap() }

    rule simpleEscapeSequence() -> char
        = "\\" !(digit() / "x" / "u" / eolChar()) c:$([_]) {
            match c.chars().next().unwrap() {
                //'b' => '\b',
                //'f' => '\f',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                //'v' => '\v',
                x  => x
            }
        }

    rule zeroEscapeSequence() -> char
        = "\\0" !digit() { 0u8 as char }

    rule hex2EscapeSequence() -> char
        = "\\x" value:$(hexDigit() hexDigit()) {
            char::from_u32(u32::from_str_radix(value, 16).unwrap()).unwrap()
        }

    rule unicodeEscapeSequence() -> char
        = "\\u{" value:$(hexDigit()+) "}" {
            char::from_u32(u32::from_str_radix(value, 16).unwrap()).unwrap()
        }

    rule eolEscapeSequence() -> char
        = "\\" eol() { '\n' }

    rule digit()
        = ['0'..='9']

    rule hexDigit()
        = ['0'..='9' | 'a'..='f' | 'A'..='F']
}}
