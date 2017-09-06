#[macro_use]
extern crate nom;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use nom::{digit};

#[derive(Debug)]
enum SyntacticKeyword {
    Else, Arrow, Define, Unquote, UnquoteSplicing,
    Expression(ExpressionKeyword)
}

#[derive(Debug)]
enum ExpressionKeyword {
    Quote, Lambda, If, Set, Begin, Cond, And, Or,
    Case, Let, LetStar, LetRec, Do, Delay, Quasiquote
}

named!(
    syntactic_keyword<SyntacticKeyword>,
    alt_complete!(
        map!(expression_keyword,      |e| SyntacticKeyword::Expression(e)) |
        map!(tag!("else"),            |_| SyntacticKeyword::Else) |
        map!(tag!("=>"),              |_| SyntacticKeyword::Arrow) |
        map!(tag!("define"),          |_| SyntacticKeyword::Define) |
        map!(tag!("unquote-splicing"),|_| SyntacticKeyword::UnquoteSplicing) |
        map!(tag!("unquote"),         |_| SyntacticKeyword::Unquote)
    )
);

named!(
    expression_keyword<ExpressionKeyword>,
    alt_complete!(
        map!(tag!("quote"),     |_| ExpressionKeyword::Quote) |
        map!(tag!("lambda"),    |_| ExpressionKeyword::Lambda) |
        map!(tag!("if"),        |_| ExpressionKeyword::If) |
        map!(tag!("set!"),      |_| ExpressionKeyword::Set) |
        map!(tag!("begin"),     |_| ExpressionKeyword::Begin) |
        map!(tag!("cond"),      |_| ExpressionKeyword::Cond) |
        map!(tag!("and"),       |_| ExpressionKeyword::And) |
        map!(tag!("or"),        |_| ExpressionKeyword::Or) |
        map!(tag!("case"),      |_| ExpressionKeyword::Case) |
        map!(tag!("letrec"),    |_| ExpressionKeyword::LetRec) |
        map!(tag!("let*"),      |_| ExpressionKeyword::LetStar) |
        map!(tag!("let"),       |_| ExpressionKeyword::Let) |
        map!(tag!("do"),        |_| ExpressionKeyword::Do) |
        map!(tag!("delay"),     |_| ExpressionKeyword::Delay) |
        map!(tag!("quasiquote"),|_| ExpressionKeyword::Quasiquote)
    )
);

named!(
    integer_literal,
    recognize!(
        do_parse!(
            opt!(tag!("-")) >>
            digit >>
            ()
        )
    )
);

named!(
    integer<i64>,
    map_res!(
        map_res!(
            integer_literal,
            std::str::from_utf8
        ),
        |s: &str| s.parse::<i64>()
    )
);

fn parse(line: &str) {
    // let res = syntactic_keyword(line.as_bytes());
    let res = integer(line.as_bytes());
    println!("Parsed {:#?}", res);
}

fn main() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                parse(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

