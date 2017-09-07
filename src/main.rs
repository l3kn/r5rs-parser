#[macro_use]
extern crate nom;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use nom::{digit, oct_digit, hex_digit};

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

fn is_bin_digit(byte: u8) -> bool {
    byte == b'0' || byte == b'1'
}

named!(bin_digit, take_while1!(is_bin_digit));

named!(sign, recognize!(opt!(one_of!("+-"))));

named!(
    integer_literal2,
    recognize!(do_parse!(sign >> bin_digit >> ()))
);

named!(
    integer_literal8,
    recognize!(do_parse!(sign >> oct_digit >> ()))
);

named!(
    integer_literal10,
    recognize!(do_parse!(sign >> digit >> ()))
);

named!(
    integer_literal16,
    recognize!(do_parse!(sign >> hex_digit >> ()))
);

named!(
    integer2<i64>,
    map_res!(
        map_res!(integer_literal2, std::str::from_utf8),
        |s| i64::from_str_radix(s, 2)
    )
);

named!(
    integer8<i64>,
    map_res!(
        map_res!(integer_literal8, std::str::from_utf8),
        |s| i64::from_str_radix(s, 8)
    )
);

named!(
    integer10<i64>,
    map_res!(
        map_res!(integer_literal10, std::str::from_utf8),
        |s| i64::from_str_radix(s, 10)
    )
);

named!(
    integer16<i64>,
    map_res!(
        map_res!(integer_literal16, std::str::from_utf8),
        |s| i64::from_str_radix(s, 16)
    )
);

named!(
    integer<i64>,
    alt!(
        preceded!(tag!("#b"), integer2) |
        preceded!(tag!("#o"), integer8) |
        preceded!(opt!(tag!("#d")), integer10) |
        preceded!(tag!("#x"), integer16)
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

