use std::env;
use std::fs;
use regex::Regex;
use inline_colorization::*;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use serde::__private::de::Borrowed;
use std::cell::RefMut;
use crate::Node::Notation;
use crate::Node::BoolLit;
use crate::Node::StringLit;
use crate::Node::IntLit;
use crate::Node::FloatLit;
use crate::Node::OperationSum;
use crate::Node::OperationEq;
use crate::Node::OperationGt;
use crate::Node::OperationNot;
use crate::Node::OperationAnd;
use crate::Node::OperationOr;
use crate::Node::OperationDivide;
use crate::Node::OperationMultiply;
use crate::Node::OperationConcat;
use crate::Node::OperationIf;
use crate::Node::CellSequence;
use crate::Node::DataCells;
use crate::Node::Sheet;
use crate::Node::Spreadsheet;
use crate::Node::Evaluator;
use core::iter;
// use std::iter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Start, Identifier, String, Int, Double, Assign, Bool, Op_Paren_o, Arr_o, Op_Paren_c, Arr_c, Colon,
    OP_Brace_o, OP_Brace_c, Semicolon, Comma, Punc, Operation_Sum, Operation_Multiply, Operation_Divide,
    Operation_And, Operation_Or, Operation_Eq, Operation_Not, Operation_Concat, Operation_Gt, Operation_If,
    Id, Data, Sheets , Notation, Submission_url,
    Eof
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,

    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {
            kind,
            literal,
        }
    }
}

#[derive(Debug)]
struct Operation {
        body: Vec<char>,
        counter: usize,
        offset: usize,
        line: usize,
        id: usize, 
        tokens: Vec<Token>, 
        buffer: String,
}


#[derive(Debug)]
pub struct Lexer {
    body: Vec<char>,
    counter: usize,
    offset: usize,
    line: usize,
    id: usize,
    tokens: Vec<Token>,
    buffer: String,
}

impl Operation {
    // add code here
    pub fn new(body: String) -> Self {

        Self { 
                body: body.chars().collect(),
                counter: 0,
                offset: 0,
                line: 1,
                id: 0,
                tokens: Vec::<Token>::new(),
                buffer: String::new()
            }

    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn new_token(&mut self, kind: TokenKind){
        self.tokens.push(Token::new(kind, self.buffer.to_owned()));
        self.reset();
    }

    fn new_int_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Int, self.buffer.to_owned()));
        self.reset();
    }

    fn new_identifier_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Identifier, self.buffer.to_owned()));
        self.reset();
    }

    fn new_bool_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Bool, self.buffer.to_owned()));
        self.reset();
    }

    fn new_double_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Double, self.buffer.to_owned()));
        self.reset();
    }

    fn new_string_token(&mut self){
        self.tokens.push(Token::new(TokenKind::String, self.buffer.to_owned()));
        self.reset();
    }

    fn new_notation_token(&mut self, buff: String){
        self.tokens.push(Token::new(TokenKind::Notation, buff));
        self.reset();
    }

    fn char_at(&self) -> char {
        *self.body.get(self.offset).unwrap()
    }

    fn peek(&self) -> Option<char> {
        let next = self.offset + 1;
        if next > self.body.len() {
            return None;
        } else {
            return Some(*self.body.get(next).unwrap());
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        // println!("{:?}", self.body);
        while self.offset < self.body.len() {
            let a = self.char_at();
            match a {
                _ if a.is_numeric() => {
                    self.lex_int();
                    self.id+=1;
                },
                '(' => {
                    self.new_token(TokenKind::Op_Paren_o);
                    self.id+=1;
                },
                ')' => {
                    self.new_token(TokenKind::Op_Paren_c);
                    self.id+=1;
                },
                '=' => {
                    self.new_token(TokenKind::Assign);
                    self.id+=1;
                },
                ',' => {
                    self.new_token(TokenKind::Comma);
                    self.id+=1;
                },
                '"' => {
                    self.lex_string();
                    self.id+=1;
                },
                '\\' => {
                    let next = self.peek();
                    if next.unwrap() == '"' {
                        self.buffer.push('\"');
                        self.offset+=1;
                        self.lex_string();
                        self.id+=1;
                    } else {
                        eprintln!("This is going to standard error!, {}", a);
                    }
                },
                '\n' => {
                    self.line += 1;
                },
                _ if a.is_alphabetic() => {
                    self.lex_ident();
                    self.id+=1;
                },
                _ => {
                    eprintln!("This is going to standard error!, {}", a);
                } 

            }
            self.offset+=1;
        }

        // self.tokens.push(Token::new(TokenKind::Eoo, "".to_owned()));
        let tok = self.tokens.clone();
        tok
    }


    fn lex_ident(&mut self){
        let mut currChar;
        while self.offset != self.body.len() {
            currChar = self.char_at();
            if(currChar.is_alphabetic()) || (currChar.is_numeric()) {
                self.buffer.push(currChar);
                self.offset+=1;
            }
            else {
                self.key_check();
                self.offset-=1;
                break;
            }
        }
        self.key_check();
        return;
    }

    fn key_check(&mut self) {
        let key = vec!["IF", "GT", "EQ", "SUM", "MULTIPLY", "CONCAT", "DIVIDE", "NOT", "AND", "OR"];
        let bools = vec!["true","false"];
        let types = vec!["int", "float", "bool","string"];
        let re = Regex::new(r"[A-Z][0-9]+").unwrap();

        for kw in key.iter() {
            if kw.eq_ignore_ascii_case(&self.buffer.to_owned()) {
                if *kw == "IF" { self.new_token(TokenKind::Operation_If); self.reset(); return; }
                if *kw == "GT" { self.new_token(TokenKind::Operation_Gt); self.reset();  return; }
                if *kw == "EQ" { self.new_token(TokenKind::Operation_Eq); self.reset();  return; }
                if *kw == "SUM" { self.new_token(TokenKind::Operation_Sum); self.reset();  return; }
                if *kw == "MULTIPLY" { self.new_token(TokenKind::Operation_Multiply);self.reset();  return; }
                if *kw == "CONCAT" { self.new_token(TokenKind::Operation_Concat); self.reset(); return; }
                if *kw == "NOT" { self.new_token(TokenKind::Operation_Not); self.reset(); return; }
                if *kw == "AND" { self.new_token(TokenKind::Operation_And); self.reset(); return; }
                if *kw == "OR" { self.new_token(TokenKind::Operation_Or); self.reset(); return; }
                if *kw == "DIVIDE" { self.new_token(TokenKind::Operation_Divide); self.reset(); return; }
            }
        }

        for kw in bools.iter() {
            if kw.eq_ignore_ascii_case(&self.buffer.to_owned()) {
                self.new_bool_token();
                return;
            }
        }

        if re.captures(&self.buffer.to_owned()).is_some() {
            self.new_notation_token(self.buffer.to_owned());
            return;
        }
    }

    fn lex_string(&mut self) {
        let mut currChar;
            self.offset+=1;

            while self.offset != self.body.len() {
                currChar = self.char_at();
                if currChar=='\r' {

                } else if currChar=='\\' {
                    self.buffer.push('\"');
                    self.offset+=1;
                    self.new_string_token();
                    break;
                } else if currChar=='\n' {
                    println!("Invalid string!, {}", currChar);
                    break;
                } else {
                    self.buffer.push(currChar);
                    self.offset+=1;
                }
            }
    }

    fn lex_int(&mut self) {
        let mut currChar;

        while self.offset != self.body.len() {
            currChar = self.char_at();
            if currChar.is_numeric() {
                self.buffer.push(currChar);
                self.offset+=1;
            }
            else if currChar.is_alphabetic() {
                println!("Invalid int!, {}", currChar);
            }
            else if currChar == '.' {
                self.buffer.push(currChar);
                self.offset+=1;
                self.lex_float();
                break;
            }
            else {
                self.new_int_token();
                    self.offset-=1;
                    break;
            }
        }
    }

    fn lex_float(&mut self) {
        let mut currChar;
        while self.offset != self.body.len() {
            currChar = self.char_at();

            if currChar.is_numeric() {
                self.buffer.push(currChar);
                self.offset+=1;
            } else if currChar.is_alphabetic() {
                println!("Invalid double!, {}", currChar);
            }
            else{
                self.new_double_token();
                self.offset-=1;
                break;
            }
        }
    }
}

impl Lexer {

    pub fn new(body: String) -> Self {

        Self { 
                body: body.chars().collect(),
                counter: 0,
                offset: 0,
                line: 1,
                id: 0,
                tokens: Vec::<Token>::new(),
                buffer: String::new()
            }

    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn new_token(&mut self, kind: TokenKind){
        self.tokens.push(Token::new(kind, self.buffer.to_owned()));
        self.reset();
    }


    fn new_identifier_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Identifier, self.buffer.to_owned()));
        self.reset();
    }

    fn new_int_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Int, self.buffer.to_owned()));
        self.reset();
    }

    fn new_bool_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Bool, self.buffer.to_owned()));
        self.reset();
    }

    fn new_double_token(&mut self){
        self.tokens.push(Token::new(TokenKind::Double, self.buffer.to_owned()));
        self.reset();
    }

    fn new_string_token(&mut self){
        self.tokens.push(Token::new(TokenKind::String, self.buffer.to_owned()));
        self.reset();
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.offset < self.body.len() {
            let a = self.char_at();
            match a {
                _ if a.is_numeric() => {
                    self.lex_int();
                    self.id+=1;
                }
                '=' => {
                    self.new_token(TokenKind::Assign);
                    self.id+=1;
                },
                '(' => {
                    self.new_token(TokenKind::Op_Paren_o);
                    self.id+=1;
                },
                ')' => {
                    self.new_token(TokenKind::Op_Paren_c);
                    self.id+=1;
                },
                '[' => {
                    self.new_token(TokenKind::Arr_o);
                    self.id+=1;
                },
                ']' => {
                    self.new_token(TokenKind::Arr_c);
                    self.id+=1;
                },
                '{' => {
                    self.new_token(TokenKind::OP_Brace_o);
                    self.id+=1;
                },
                '}' => {
                    self.new_token(TokenKind::OP_Brace_c);
                    self.id+=1;
                },                
                '.' => {
                    self.new_token(TokenKind::Colon);
                    self.id+=1;
                },
                ',' => {
                    self.new_token(TokenKind::Comma);
                    self.id+=1;
                },
                ';' => {
                    self.new_token(TokenKind::Semicolon);
                    self.id+=1;
                },
                ':' => {
                    self.new_token(TokenKind::Punc);
                    self.id+=1;
                },
                '"' => {
                    let next = self.peek();
                    if next.unwrap() == '=' {
                        self.lex_op()
                    } else {
                        self.lex_string();
                        self.id+=1;
                    }
                },
                _ if a.is_alphabetic() => {
                    self.lex_ident();
                    self.id+=1;
                },
                '\n' => {
                    self.line += 1;
                },
                _ => {
                    eprintln!("This is going to standard error!, {}", a);
                } 

            }
            self.offset+=1;
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".to_owned()));

        let tok =  self.tokens.clone();
        tok
    }

    fn char_at(&self) -> char {
        *self.body.get(self.offset).unwrap()
    }

    fn peek(&self) -> Option<char> {
        let next = self.offset + 1;
        if next > self.body.len() {
            return None;
        } else {
            return Some(*self.body.get(next).unwrap());
        }
    }

    fn lex_ident(&mut self){
        let mut currChar;
        while self.offset != self.body.len() {
            currChar = self.char_at();
            if(currChar.is_alphabetic()) || (currChar.is_numeric()) || (currChar == '_') {
                self.buffer.push(currChar);
                self.offset+=1;
            }
            else {
                self.key_check();
                self.offset-=1;
                break;
            }
        }
    }

    fn key_check(&mut self) {
        let key = vec!["data", "sheets", "id", "submissionUrl"];
        let bools = vec!["true","false"];
        let types = vec!["int", "float", "bool","string"];
        let re = Regex::new(r"[A-Z][0-9]+").unwrap();

        for kw in key.iter() {
            if kw.eq_ignore_ascii_case(&self.buffer.to_owned()) {
                if *kw == "data" { self.new_token(TokenKind::Data); self.reset(); return; }
                if *kw == "sheets" { self.new_token(TokenKind::Sheets); self.reset(); return; }
                if *kw == "id" { self.new_token(TokenKind::Id); self.reset(); return; }
                if *kw == "submissionUrl" { self.new_token(TokenKind::Submission_url); self.reset(); return; }
            }
        }

        for kw in bools.iter() {
            if kw.eq_ignore_ascii_case(&self.buffer.to_owned()) {
                self.new_bool_token();
                return;
            }
        }
        self.new_identifier_token();
    }

    fn lex_string(&mut self) {
        let mut currChar;
            self.offset+=1;

            while self.offset != self.body.len() {
                currChar = self.char_at();
                if currChar=='\r' {

                }else if currChar=='"' {
                    self.new_string_token();
                    break;
                }else if currChar=='\n' {
                    println!("Invalid string!, {}", currChar);
                    break;
                }else if currChar=='\\' {
                    println!("Invalid string!, {}", currChar);
                    break;
                } else {
                    self.buffer.push(currChar);
                    self.offset+=1;
                }
            }
    }

    fn lex_op(&mut self) {
        let mut currChar;
        self.id+=1;
        self.offset+=1;

        while self.offset != self.body.len() {
            currChar = self.char_at();
            if currChar=='"' {
                self.check_inside();
                self.reset();
                break;
            }else if currChar=='\\' {
                let next = self.peek();
                if next.unwrap() == '"' {
                    self.buffer.push(currChar);
                    self.offset+=1;
                    self.buffer.push(next.unwrap());
                    self.offset+=1;
                } else {
                    self.buffer.push(currChar);
                    self.offset+=1;
                    break;
                }
            } else if currChar=='\n' {
                println!("Invalid string!, {}", currChar);
                break;
            } else {
                self.buffer.push(currChar);
                self.offset+=1;
            }
        }
    }

    fn check_inside(&mut self) {
        let cvec = self.buffer.to_owned();
        let mut op = Operation::new(cvec); 
        let tok = op.lex();
        for element in tok.iter() {
            self.tokens.push(element.clone());
        }
    }

    fn lex_int(&mut self) {
        let mut currChar;

        while self.offset != self.body.len() {
            currChar = self.char_at();
            if currChar.is_numeric() {
                self.buffer.push(currChar);
                self.offset+=1;
            }
            else if currChar.is_alphabetic() {
                println!("Invalid int!, {}", "awesome");
            }
            else if currChar == '.' {
                self.buffer.push(currChar);
                self.offset+=1;
                self.lex_float();
                break;
            }
            else {
                self.new_int_token();
                self.offset-=1;
                break;
            }
        }
    }

    fn lex_float(&mut self) {
        let mut currChar;
        while self.offset != self.body.len() {
            currChar = self.char_at();

            if currChar.is_numeric() {
                self.buffer.push(currChar);
                self.offset+=1;
            } else if currChar.is_alphabetic() {
                println!("Invalid double!, {}", "awesome");
            }
            else{
                self.new_double_token();
                self.offset-=1;
                break;
            }
        }
    }

}

#[derive(Debug, PartialEq)]
pub enum Response {
    DataCells(Box<ResultsData>),
    Sheet(Box<ResultsSheet>),
    Spreadsheet(Box<SpreadsheetResponseNode>),
    Evaluator(Box<EvaluatorResponseNode>),
}
#[derive(Debug, PartialEq)]
pub struct EvaluatorResponseNode {
    pub values: Box<Option<Response>>,
}
#[derive(Debug, PartialEq)]
pub struct SpreadsheetResponseNode {
    pub values: Vec<Box<Option<Response>>>,
}

#[derive(Debug, PartialEq)]
pub struct ResultsSheet {
    pub id: String,
    pub values: Box<Option<Response>>,
}

#[derive(Debug, PartialEq)]
pub struct ResultsData {
    pub data: String,
    pub cells: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    BoolLit(LitNode),
    StringLit(LitNode),
    IntLit(LitNode),
    FloatLit(LitNode),
    Notation(NotationNode),
    OperationSum(SumNode),
    OperationEq(EqNode),
    OperationGt(GtNode),
    OperationMultiply(MultiplyNode),
    OperationAnd(AndNode),
    OperationNot(NotNode),
    OperationOr(OrNode),
    OperationIf(IfNode),
    OperationConcat(ConcatNode),
    OperationDivide(DivideNode),
    CellSequence(CellSequenceNode),
    DataCells(DataNode),
    Sheet(SheetNode),
    Spreadsheet(SpreadsheetNode),
    Evaluator(EvaluatorNode),
    // Notation(Box<BinExpr>)
}


#[derive(Debug, PartialEq)]
pub struct EvaluatorNode {
    pub url: Token,
    pub values: Box<[Node]>,
}

#[derive(Debug, PartialEq)]
pub struct SpreadsheetNode {
    pub values: Box<[Node]>,
}

#[derive(Debug, PartialEq)]
pub struct SheetNode {
    pub id: Token,
    pub values: Box<[Node]>,
}

#[derive(Debug, PartialEq)]
pub struct DataNode {
    pub values: Box<[Node]>,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct CellSequenceNode {
    pub values: Box<[Node]>,
    pub mem: Option<Vec<String>>,
}

impl CellSequenceNode {
    pub fn add_mem(&mut self, mem:Vec<String>) {
        self.mem = Some(mem.clone());
    }
}

#[derive(Debug, PartialEq)]
pub struct NotationNode {
    pub token: Token,
    pub slot: String,
}

#[derive(Debug, PartialEq)]
pub struct SumNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct EqNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct NotNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct OrNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct DivideNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct ConcatNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct MultiplyNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct AndNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct GtNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct IfNode {
    pub values: Box<[Node]>,
    pub token: Token,
    pub slot: String,
    pub mem: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct LitNode {
    pub token: Token,
    pub slot: String,
}


#[derive(Debug)]
pub struct Interpreter {
    pub mem: Vec<String>,
    pub val: Vec<String>,
}

impl Interpreter {
    pub fn new(mem: Vec<String>, val: Vec<String>) -> Self {

        Self { 
                mem: mem,
                val: val,
            }
    }

    pub fn exec_one(&mut self, frame: usize, one: Vec<String>, mem: Vec<String>, val: Vec<String>) -> Vec<String> {
        let mut i = 0;
        let re = Regex::new(r"[A-Z][0-9]+").unwrap();
        let mut stack:Vec<String> = Vec::new();

        while i < one.len() {

            let op = one.get(i).unwrap();
            match op.as_str() {
                "SUM" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "CONCAT" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "OR" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "IF" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "AND" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "NOT" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "GT" => {
                    i+=1;

                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "EQ" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "DIVIDE" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                "MULTIPLY" => {
                    i+=1;
                    let mut f = frame.clone();
                    let mut stack_mem:Vec<String> = Vec::new();
                    let mut stack_val:Vec<String> = Vec::new();
                    while f < mem.len() {
                        stack_mem.push(mem.get(f).unwrap().to_string());
                        stack_val.push(val.get(f).unwrap().to_string());
                        f+=1;
                    }
                    let r = self.exec(stack_mem, stack_val);
                    stack.push(r.get(0).unwrap().to_string());
                },
                _ if op.parse::<f64>().is_ok() => {
                    stack.push(op.to_string());
                    i+=1;
                },
                _ if re.captures(op).is_some() => {
                    let mut index = 0;
                    while index < self.mem.len() {
                        if self.mem.get(index).unwrap() == op {
                            break;
                        }
                        index+=1;
                    }
                    stack.push(self.val.get(index).unwrap().to_string());
                    i+=1;
                },
                _ if op.chars().all(char::is_alphanumeric)|| op.contains(char::is_whitespace) || op.contains("\"") || op.contains(",")=> {
                    stack.push(op.to_string());
                    i+=1;
                },
                _ => {
                    i+=1;
                }
            }
        }
      
        stack
    }

    pub fn exec(&mut self, mem: Vec<String>, val: Vec<String>) -> Vec<String> {
        let mut i = 0;
        let mut res1:Vec<String> = Vec::new();
        let mut res:Vec<String> = Vec::new();
        let re = Regex::new(r"[A-Z][0-9]+").unwrap();
        while i < mem.len() {

            let op = val.get(i).unwrap();
            match op.as_str() {
                "SUM" => {
                    i+=1;
                    let mut sum_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_SUM" {
                            break;
                        } else {
                            sum_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), sum_end, mem.clone(), val.clone());
                    let mut sum_result = 0 as f64;
                    for x in stack {
                        if x.parse::<f64>().is_ok() {
                            let a = x.parse::<f64>().unwrap();
                            sum_result = sum_result + a;
                        } else {
                            res.push("ERROR: type does not match".to_string());
                            break;
                        }
                    }
                    i = j;
                    res.push(sum_result.to_string());
                },
                "CONCAT" => {
                    i+=1;
                    let mut concat_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_CONCAT" {
                            break;
                        } else {
                            concat_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), concat_end, mem.clone(), val.clone());
                    let mut concat_res = "".to_string();
                    for x in stack {
                        if x.chars().all(char::is_alphanumeric )|| x.contains(char::is_whitespace) || x.contains("\"") || x.contains(",") {
                            concat_res = concat_res + &x;
                        } else {
                            res.push("ERROR: type does not match".to_string());
                            break;
                        }
                    }
                    i = j;
                    let s = concat_res.replace("\"", "");
                    res.push(s);
                },
                "OR" => {
                    i+=1;
                    let mut or_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_OR" {
                            break;
                        } else {
                            or_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), or_end, mem.clone(), val.clone());
                    let mut boo = false;
                    let mut bbb = 1;
                    for x in stack {
                        if x != "true" && x != "false" {
                            bbb = -1;
                        }
                        if x == "true" {
                            boo = true; 
                        }
                    }
                    if bbb > 0 {
                        res.push(boo.to_string());
                    } else {
                        res.push("ERROR: type does not match".to_string());
                    }
                    i = j;
                },
                "IF" => {
                    i+=1;
                    let mut if_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_OR" {
                            break;
                        } else {
                            if_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), if_end, mem.clone(), val.clone());
                    let mut pop = stack.get(0).unwrap().to_string();
                    if pop == "true" {
                        res.push(stack.get(1).unwrap().to_string());
                    } else if pop == "false" {
                        res.push(stack.get(2).unwrap().to_string());
                    } else {
                        res.push("ERROR: type does not match".to_string());
                    }
                    i = j;
                },
                "AND" => {
                    i+=1;
                    let mut and_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_AND" {
                            break;
                        } else {
                            and_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), and_end, mem.clone(), val.clone());
                    let mut pop = stack.get(0).unwrap().to_string();
                    for x in stack {
                        if x == "false" {
                            res.push("false".to_string());
                        } else if x != "true" && x != "false" {
                            res.push("ERROR: type does not match".to_string());
                        } else {
                            res.push("true".to_string());
                        }
                    }
                    i = j;
                },
                "NOT" => {
                    i+=1;
                    let mut not_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_NOT" {
                            break;
                        } else {
                            not_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), not_end, mem.clone(), val.clone());
                    let mut x = stack.get(0).unwrap().to_string();
                    if x != "true" && x != "false" {
                        res.push("ERROR: type does not match".to_string());
                    } else if x == "true" {
                        res.push("true".to_string()); 
                    } else {
                        res.push("false".to_string()); 
                    }
                    i = j;
                },
                "GT" => {
                    i+=1;
                    let mut gt_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_GT" {
                            break;
                        } else {
                            gt_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), gt_end, mem.clone(), val.clone());
                    let mut gt_result = "false".to_string();
                    let mut pop = stack.get(0).unwrap().to_string();
                    let mut pop2 = stack.get(1).unwrap().to_string();
                    if pop.parse::<f64>().is_ok() && pop2.parse::<f64>().is_ok() {
                        if pop > pop2 {
                            res.push("true".to_string());
                        } else {
                            res.push("false".to_string());
                        }
                    } else {
                        res.push("ERROR: type does not match".to_string());
                        break;
                    }
                    i = j;
                },
                "EQ" => {
                    i+=1;
                    let mut eq_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_EQ" {
                            break;
                        } else {
                            eq_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), eq_end, mem.clone(), val.clone());
                    let mut pop = stack.get(0).unwrap().to_string();
                    let mut pop2 = stack.get(1).unwrap().to_string();
                    if pop == pop2 {
                        res.push("true".to_string());
                    } else {
                        res.push("false".to_string());
                    }
                    i = j;
                },
                "DIVIDE" => {
                    i+=1;
                    let mut div_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_DIV" {
                            break;
                        } else {
                            div_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), div_end, mem.clone(), val.clone());
                    let mut pop = stack.get(0).unwrap().to_string();
                    let mut pop2 = stack.get(1).unwrap().to_string();
                    if pop.parse::<f64>().is_ok() && pop2.parse::<f64>().is_ok() {
                        let a = pop.parse::<f64>().unwrap();
                        let a2 = pop2.parse::<f64>().unwrap();
                        let mut div_result = a / a2;
                        res.push(div_result.to_string())
                    } else {
                        res.push("ERROR: type does not match".to_string());
                        break;
                    }

                    i = j;
                },
                "MULTIPLY" => {
                    i+=1;
                    let mut mul_end:Vec<String> = Vec::new();
                    let mut j = i;
                    while j < mem.len() {
                        if mem.get(j).unwrap().as_str() == "END_MUL" {
                            break;
                        } else {
                            mul_end.push(val.get(j).unwrap().to_string());
                        }
                        j+=1;
                    }
                    let stack = self.exec_one(i.clone(), mul_end, mem.clone(), val.clone());
                    let mut mul_result = 1 as f64;
                    for x in stack {
                        if x.parse::<f64>().is_ok() {
                            let a = x.parse::<f64>().unwrap();
                            mul_result = mul_result * a;
                        } else {
                            res.push("ERROR: type does not match".to_string());
                            break;
                        }
                    }
                    i = j;
                    res.push(mul_result.to_string());
                },
                _ if op.parse::<f64>().is_ok() => {
                    res.push(op.to_string());
                    i+=1;
                },
                _ if re.captures(op).is_some() => {
                    res.push(op.to_string());
                    i+=1;
                },
                _ if op.chars().all(char::is_alphanumeric)=> {
                    res.push(op.to_string());
                    i+=1;
                },
                _ => {
                    res.push(op.to_string());
                    i+=1;
                }
            }

            // i+=1;
        }
        let mut z = 0;
        while z < mem.len() {
            res = self.exec_one(i.clone(), res, mem.clone(), val.clone());
            z+=1;
        }
        
        return res
    }

}

impl Node {

    pub fn resolver(&self)-> Box<Option<Response>>{
        match self {
            Node::Evaluator(t) => { 
                let ret = Response::Evaluator(Box::new(EvaluatorResponseNode {values: self.spreadsheet_resolver(&t.values)}));
                return Box::new(Some(ret))
            },
            _=> println!("{color_red}nothing")
        }
        return Box::new(None)
    }

    pub fn spreadsheet_resolver(&self, node:&[Node])-> Box<Option<Response>> {
        let mut sheets:Vec<Box<Option<Response>>> = Vec::new();
        for t in node {
            match t {
                Node::Spreadsheet(t) => {
                    sheets.push(self.sheet_resolver(&t.values));
                },
                _=> println!("{color_red}nothing")
            }
        }
        return Box::new(Some(Response::Spreadsheet(Box::new(SpreadsheetResponseNode {values: sheets}))))
    }

    pub fn sheet_resolver(&self, node:&[Node])-> Box<Option<Response>> {
        let mut sheets:Vec<Box<Option<Response>>> = Vec::new();
        for t in node {
            match t {
                Node::Sheet(t) => {
                    sheets.push(Box::new(Some(Response::Sheet(Box::new(ResultsSheet {id: t.id.literal.clone(), values: self.resolve(&t.values)})))));
                },
                _=> println!("{color_red}nothing")
            }
        }
        return Box::new(Some(Response::Spreadsheet(Box::new(SpreadsheetResponseNode {values: sheets}))))
    }

    pub fn resolve(&self, node:&[Node]) -> Box<Option<Response>>{
        let regex = Regex::new(r"[a-z][0-9]+").unwrap();
        for t in node {
            match t {
                Node::DataCells(t) => {
                    let a = self.interpreter(&t.values);
                    let mut mem1:Vec<String> = Vec::new();
                    let mut val1:Vec<String> = Vec::new();
                    for mut i in a {
                        mem1.append(&mut i.mem);
                        val1.append(&mut i.val);
                    }
                    let mut interp = Interpreter::new(mem1.clone(), val1.clone());
                    let mut re = interp.exec(mem1.clone(), val1.clone());
                    
                    let mut index = 0;
                    while index < re.len() {
                        let x = re.get(index).unwrap();
                        if x == "0000" {
                            re.remove(index);
                            index-=1;
                        }
                        index+=1;
                    }
                    index = 0;
                    while index < mem1.len() {
                        let x = mem1.get(index).unwrap();
                        if regex.captures(x).is_some() || x =="END_SUM" || x == "END_MUL"|| x == "END_DIV"|| x == "END_OR"|| x == "END_EQ"|| x == "END_CONCAT"|| x == "END_NOT"|| x == "END_GT" || x == "END_IF"|| x == "END_AND"{
                            mem1.remove(index);
                            index-=1;
                        }
                        index+=1;
                    }
                    return Box::new(Some(Response::DataCells(Box::new(ResultsData {data: "data".to_string(), cells: re}))))
                },
                Node::CellSequence(t) => { 
                    self.resolve(&t.values);
                },
                Node::OperationDivide(t) => {
                    self.eval_expr(&t.values); 
                },
                Node::OperationConcat(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationIf(t) => {
                    self.eval_expr(&t.values);  
                },
                Node::OperationOr(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationNot(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationAnd(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationMultiply(t) => {
                    self.eval_expr(&t.values); 
                },
                Node::OperationGt(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationEq(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::OperationSum(t) => { 
                    self.eval_expr(&t.values); 
                },
                Node::Notation(t) => { 

                },
                Node::FloatLit(t) => { 

                },
                Node::IntLit(t) => { 

                },
                Node::StringLit(t) => { 

                },
                Node::BoolLit(t) => { 

                },
                _=> println!("{color_red}nothing")
            }
        }
        Box::new(None)
    }


    pub fn interpreter(&self, node:&[Node])-> Vec<Mem>{
        let mut mem:Vec<Mem> = Vec::new();
        for value in node {
            match value {
                Node::CellSequence(t) => {
                    let a= self.eval_expr(&t.values);
                    mem.push(a);
                },
                _ => println!("{color_red}nothing")
            }
        }
        mem
    }

    pub fn eval_expr(&self, node:&[Node]) -> Mem  {
        let mut mem:Vec<String> = Vec::new();
        let mut val:Vec<String> = Vec::new();
        for t in node {
            match t {
                Node::OperationDivide(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_DIV".to_string()); val.push("0000".to_string());
                },
                Node::OperationConcat(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_CONCAT".to_string()); val.push("0000".to_string());
                },
                Node::OperationIf(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_IF".to_string()); val.push("0000".to_string());
                },
                Node::OperationOr(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_OR".to_string()); val.push("0000".to_string());
                },
                Node::OperationNot(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_NOT".to_string()); val.push("0000".to_string());
                },
                Node::OperationAnd(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_AND".to_string()); val.push("0000".to_string());
                },
                Node::OperationMultiply(t) => { 
                    mem.push(t.slot.clone());val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    } 
                    mem.push("END_MUL".to_string()); val.push("0000".to_string());
                },
                Node::OperationGt(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    } 
                    mem.push("END_GT".to_string()); val.push("0000".to_string());
                },
                Node::OperationEq(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_EQ".to_string()); val.push("0000".to_string());
                },
                Node::OperationSum(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                    let a = self.eval_expr(&t.values);
                    for d in a.mem {
                        mem.push(d);
                    }
                    for d in a.val {
                        val.push(d);
                    }
                    mem.push("END_SUM".to_string()); val.push("0000".to_string());
                },
                Node::Notation(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                },
                Node::FloatLit(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                },
                Node::IntLit(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                },
                Node::StringLit(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                },
                Node::BoolLit(t) => { 
                    mem.push(t.slot.clone()); val.push(t.token.literal.clone());
                },
                _=> println!("DEAD END")
            }
        }

        let values = Mem::new(mem, val); 
        values
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    offset: usize,
    curr_token: Token,
    slot: usize,
    chars: Vec<char>,
    tiny_chars: Vec<char>,
    stack_slot: usize,
    flag: bool,
}

impl Parser {
        
    pub fn new(tokens: Vec<Token>) -> Self {
            
        Self {
            tokens: tokens,
            offset: 0,
            curr_token: Token::new(TokenKind::Start, "".to_string()),
            slot: 0,
            chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            tiny_chars: "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            stack_slot:1,
            flag: false,
        }
    }

    fn current(&mut self) -> TokenKind {
        self.curr_token = self.tokens.get(self.offset).unwrap().clone();
        self.tokens.get(self.offset).unwrap().kind
    }


    fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        if self.current() == kind {
            self.offset += 1;
            return Some(self.curr_token.clone());
        }
        else{
            println!("{color_red}expected {:?}, got {:?}", kind, self.curr_token);
            None
        }
    }

    pub fn parse_all(&mut self) -> Option<Node>  {
        self.expect(TokenKind::OP_Brace_o);
        self.expect(TokenKind::String);
        self.expect(TokenKind::Punc);
        let url =  self.expect(TokenKind::String).unwrap();
        self.expect(TokenKind::Comma);
        let sheets = self.expect(TokenKind::String).unwrap();
        self.expect(TokenKind::Punc);
        let mut args: Vec<Node> = vec![];
        while self.current() != TokenKind::Eof {
            args.push(self.parse_all_sheets().unwrap());
            self.expect(TokenKind::OP_Brace_c);
        }

        println!("Parser reached EOF");
        return Some(Evaluator(EvaluatorNode { url: url, values: args.into_boxed_slice(), }))
    }

    fn parse_all_sheets(&mut self) -> Option<Node> {
        let mut args: Vec<Node> = vec![];

        self.expect(TokenKind::Arr_o);

        if self.current() != TokenKind::Arr_c { 
            args.push(self.parse_sheet().unwrap());
            while self.current() != TokenKind::Arr_c {
                self.expect(TokenKind::Comma);
                if self.current() == TokenKind::OP_Brace_o {
                    args.push(self.parse_sheet().unwrap());
                }
            }
        }

        self.expect(TokenKind::Arr_c);
        return Some(Spreadsheet(SpreadsheetNode {values: args.into_boxed_slice(), }))
    }

    fn parse_sheet(&mut self) -> Option<Node> {
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::OP_Brace_o);
        self.expect(TokenKind::String);
        self.expect(TokenKind::Punc);
        let sheet_id = self.expect(TokenKind::String).unwrap();
        self.expect(TokenKind::Comma);
        self.expect(TokenKind::String);
        self.expect(TokenKind::Punc);
        args.push(self.parse_sheet_data().unwrap());     

        self.expect(TokenKind::OP_Brace_c);
        return Some(Sheet(SheetNode { id: sheet_id, values: args.into_boxed_slice(), }))
    }

    fn parse_sheet_data(&mut self) -> Option<Node> {
        self.expect(TokenKind::Arr_o);
        let mut args: Vec<Node> = vec![];
        if self.current() == TokenKind::Arr_o {
            args.push(self.parse_expr_sequence().unwrap());     
        }

        if self.current() != TokenKind::Arr_c { 
            while self.current() != TokenKind::Arr_c {
                self.expect(TokenKind::Comma);
                if self.current() == TokenKind::Arr_o {
                    // self.expect(TokenKind::Arr_o);
                    self.stack_slot+=1;
                    args.push(self.parse_expr_sequence().unwrap());     
                }
            }
        }

        self.expect(TokenKind::Arr_c); self.reset_stack_slot();
        return Some(DataCells(DataNode {values: args.into_boxed_slice(), mem: None}))
    }

    fn peek2(&mut self, type0: TokenKind, type1: TokenKind) -> Option<bool> {
        let token0 = self.tokens.get(self.offset).unwrap().clone();
        let next = self.offset + 1;
        if next > self.tokens.len() {
            return None;
        } else {
            let token1 = self.tokens.get(self.offset+1).unwrap().clone();
            return Some((token0.kind == type0) && (token1.kind == type1));
        }
    }

    fn reset(&mut self) {
        self.slot = 0;
    }

    fn reset_stack_slot(&mut self) {
        self.stack_slot = 1;
    }

    fn parse_expr_sequence(&mut self) -> Option<Node> {
        self.expect(TokenKind::Arr_o);
        let mut args: Vec<Node> = vec![];
        if self.current() == TokenKind::Assign {
            self.expect(TokenKind::Assign);
            args.push(self.parse_operation().unwrap());     
        } else if self.current() != TokenKind::Arr_c {
            args.push(self.parse_operation().unwrap());
        }

        if self.current() != TokenKind::Arr_c { 
            while self.current() != TokenKind::Arr_c {
                self.expect(TokenKind::Comma);
                if self.current() == TokenKind::Assign {
                    self.expect(TokenKind::Assign);
                    args.push(self.parse_operation().unwrap());     
                } else {
                    args.push(self.parse_operation().unwrap());
                }
            }
        }

        self.expect(TokenKind::Arr_c); self.reset();
        return Some(CellSequence(CellSequenceNode {values: args.into_boxed_slice(), mem: None}))
    }

    fn parse_operation(&mut self) -> Option<Node> {
        match self.current() {
            TokenKind::Operation_Eq => { return self.parse_eq();},
            TokenKind::Operation_Gt => { return self.parse_gt();},
            TokenKind::Operation_If => { return self.parse_if();},
            TokenKind::Operation_And => { return self.parse_and();},
            TokenKind::Operation_Sum => { return self.parse_sum();},
            TokenKind::Operation_Or => { return self.parse_or();},
            TokenKind::Operation_Concat => { return self.parse_concat();},
            TokenKind::Operation_Multiply => { return self.parse_multilply();},
            TokenKind::Operation_Divide => { return self.parse_divide();},
            TokenKind::Operation_Not => { return self.parse_not();},
            TokenKind::Int => {
                self.slot+=1;
                let val = self.expect(TokenKind::Int).unwrap(); 
                return Some(IntLit(LitNode { token: val, slot: self.convertation(), }))
            },
            TokenKind::String => {
                self.slot+=1;
                let val = self.expect(TokenKind::String).unwrap();  
                return Some(StringLit(LitNode { token: val, slot: self.convertation(),  }))  
            },
            TokenKind::Double => {
                self.slot+=1;
                let val = self.expect(TokenKind::Double).unwrap(); 
                return Some(FloatLit(LitNode { token: val, slot: self.convertation(),  }))
            },
            TokenKind::Bool => {
                self.slot+=1;
                let val = self.expect(TokenKind::Bool).unwrap(); 
                return Some(BoolLit(LitNode {token: val, slot: self.convertation(), }))
            },
            TokenKind::Notation => {
                self.slot+=1;
                let val = self.expect(TokenKind::Notation).unwrap(); 
                let notation_slot = val.literal.clone();
                return Some(Notation(NotationNode {token: val, slot: self.convertation(), }))
            },
            _ => {
                println!("{color_red}bad node {:?}", self.current()); 
                return None
            }
        }   
    }

    fn convertation(&mut self) -> String {
        if self.flag == true {
            let s: String = self.get_tiny_char().to_string();
            let s1: String = self.stack_slot.to_string();
            let ss = s+&s1;
            return ss
        } else {
            let s: String = self.get_char().to_string();
            let s1: String = self.stack_slot.to_string();
            let ss = s+&s1;
            return ss
        }
    }

    fn get_char(&mut self) -> char {
        let a = self.slot -1;
        *self.chars.get(a).unwrap()
    }


    fn get_tiny_char(&mut self) -> char {
        let a = self.slot -1;
        *self.tiny_chars.get(a).unwrap()
    }

    fn parse_eq(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        self.flag = true;
        // self.reset();
        let sum = self.expect(TokenKind::Operation_Eq).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        self.flag = false;
        // let term_back = self.slot;
        self.slot = term;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationEq(EqNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_gt(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Gt).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        self.flag = false;
        let term_back = self.slot;
        self.slot = term;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationGt(GtNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_if(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_If).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationIf(IfNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        return Some(ret)
    }

    fn parse_and(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_And).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationAnd(AndNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }
    fn parse_not(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Not).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationNot(NotNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(), mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_or(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Or).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationOr(OrNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(), mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_divide(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Divide).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationDivide(DivideNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_concat(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Concat).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        self.flag = false;
        let term_back = self.slot;
        self.slot = term;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationConcat(ConcatNode { values: args.into_boxed_slice(), token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_sum(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Sum).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationSum(SumNode {values: args.into_boxed_slice(),token: sum, slot: self.convertation(),mem: None});
        // self.slot = term_back;
        return Some(ret)
    }

    fn parse_multilply(&mut self) -> Option<Node> {
        self.slot+=1;
        let term = self.slot;
        // self.reset();
        self.flag = true;
        let sum = self.expect(TokenKind::Operation_Multiply).unwrap(); 
        let mut args: Vec<Node> = vec![];
        self.expect(TokenKind::Op_Paren_o);
        if self.current() != TokenKind::Op_Paren_c { 
            args.push(self.parse_operation().unwrap());
            while self.current() != TokenKind::Op_Paren_c {
                self.expect(TokenKind::Comma);
                args.push(self.parse_operation().unwrap());
            }
        }
        let term_back = self.slot;
        self.slot = term;
        self.flag = false;
        self.expect(TokenKind::Op_Paren_c);
        let ret = Node::OperationMultiply(MultiplyNode {values: args.into_boxed_slice(),token: sum, slot: self.convertation(), mem: None});
        // self.slot = term_back;
        return Some(ret)
    }


    fn parse_notation(&mut self) -> Option<Node> {
        self.expect(TokenKind::Assign);
        let val = self.expect(TokenKind::Notation).unwrap();
        let notation_slot = val.literal.clone();
        return Some(Notation(NotationNode {
            token: val, slot: self.convertation(),
        }))
    }
}

#[derive(Debug)]
pub struct Mem {
    pub mem: Vec<String>,
    pub val: Vec<String>,
}

impl Mem {
    pub fn new(mem: Vec<String>, val: Vec<String>) -> Self {

        Self { 
                mem: mem,
                val: val,
            }

    }
}

