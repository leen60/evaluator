mod app;
use std::env;
use crate::app::*;
use reqwest::Error;
use std::cell::RefCell;

async fn get_request() -> Result<(), Error> {
    let response = reqwest::get("").await?;
    println!("Status: {}", response.status());

    let body = response.text().await.unwrap();
    let mut lexer = Lexer::new(body); 
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse_all().unwrap();
    let response = prog.resolver();
    dbg!(&response);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
    get_request().await?;
    Ok(())
}