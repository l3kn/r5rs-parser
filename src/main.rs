#[macro_use]
extern crate nom;
extern crate rustyline;

use std::str;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use nom::{digit, oct_digit, hex_digit, anychar};

#[derive(Debug, PartialEq)]
enum SyntacticKeyword {
    Else, Arrow, Define, Unquote, UnquoteSplicing,
    Expression(ExpressionKeyword)
}

#[derive(Debug, PartialEq)]
enum ExpressionKeyword {
    Quote, Lambda, If, Set, Begin, Cond, And, Or,
    Case, Let, LetStar, LetRec, Do, Delay, Quasiquote
}

named!(
    syntactic_keyword<SyntacticKeyword>,
    alt_complete!(
        expression_keyword       => { |e| SyntacticKeyword::Expression(e) } |
        tag!("else")             => { |_| SyntacticKeyword::Else } |
        tag!("=>")               => { |_| SyntacticKeyword::Arrow } |
        tag!("define")           => { |_| SyntacticKeyword::Define } |
        tag!("unquote-splicing") => { |_| SyntacticKeyword::UnquoteSplicing } |
        tag!("unquote")          => { |_| SyntacticKeyword::Unquote }
    )
);

named!(
    expression_keyword<ExpressionKeyword>,
    alt_complete!(
        tag!("quote")      => { |_| ExpressionKeyword::Quote } |
        tag!("lambda")     => { |_| ExpressionKeyword::Lambda } |
        tag!("if")         => { |_| ExpressionKeyword::If } |
        tag!("set!")       => { |_| ExpressionKeyword::Set } |
        tag!("begin")      => { |_| ExpressionKeyword::Begin } |
        tag!("cond")       => { |_| ExpressionKeyword::Cond } |
        tag!("and")        => { |_| ExpressionKeyword::And } |
        tag!("or")         => { |_| ExpressionKeyword::Or } |
        tag!("case")       => { |_| ExpressionKeyword::Case } |
        tag!("letrec")     => { |_| ExpressionKeyword::LetRec } |
        tag!("let*")       => { |_| ExpressionKeyword::LetStar } |
        tag!("let")        => { |_| ExpressionKeyword::Let } |
        tag!("do")         => { |_| ExpressionKeyword::Do } |
        tag!("delay")      => { |_| ExpressionKeyword::Delay } |
        tag!("quasiquote") => { |_| ExpressionKeyword::Quasiquote }
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

named!(
    boolean<bool>,
    alt!(
        tag!("#t") => { |_| true } |
        tag!("#f") => { |_| false }
    )
);

named!(
    character<char>,
    preceded!(
        tag!("#\\"),
        alt_complete!(
            tag!("space") => { |_| ' ' } |
            tag!("newline") => { |_| '\n' } |
            anychar
        )
    )
);

#[derive(Debug, PartialEq)]
enum Token {
    // Keyword(SyntacticKeyword),
    Number(i64),
    Boolean(bool),
    Character(char),
    String(String),
    Identifier(String),
    LBracket, RBracket, HashBracket,
    Quote, Quasiquote,
    Unquote, UnquoteSplicing,
    Dot
}

named!(
    token<Token>,
    alt_complete!(
        // syntactic_keyword => { |kw| Token::Keyword(kw) } |
        integer           => { |i| Token::Number(i) } |
        boolean           => { |b| Token::Boolean(b) } |
        character         => { |c| Token::Character(c) } |
        string            => { |s| Token::String(s) } |
        identifier        => { |s| Token::Identifier(s) } |
        tag!("(")         => { |_| Token::LBracket } |
        tag!(")")         => { |_| Token::RBracket } |
        tag!("#(")        => { |_| Token::HashBracket } |
        tag!("'")         => { |_| Token::Quote } |
        tag!("`")         => { |_| Token::Quasiquote } |
        tag!(",@")        => { |_| Token::UnquoteSplicing } |
        tag!(",")         => { |_| Token::Unquote } |
        tag!(".")         => { |_| Token::Dot }
    )
);

named!(string<String>,
    delimited!(tag!("\""), string_content, tag!("\""))
);

fn to_s(i: Vec<u8>) -> String {
  String::from_utf8_lossy(&i).into_owned()
}

named!(
    string_content<String>,
    map!(
        escaped_transform!(
            take_until_either!("\"\\"),
            '\\',
            alt!(
                tag!("\\") => { |_| &b"\\"[..] } |
                tag!("\"") => { |_| &b"\""[..] } |
                tag!("n") => { |_| &b"\n"[..] } |
                tag!("r") => { |_| &b"\r"[..] } |
                tag!("t") => { |_| &b"\t"[..] }
            )
        ),
        to_s
    )
);

named!(letter<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
named!(single_digit<char>, one_of!("0123456789"));
named!(special_initial<char>, one_of!("!$%&*/:<=>?^_~"));
named!(special_subsequent<char>, one_of!("+-.@"));

named!(initial<char>, alt!(letter | special_initial));
named!(subsequent<char>, alt!(initial | single_digit | special_subsequent));

named!(
    common_identifier,
    recognize!(
        do_parse!(initial >> many0!(subsequent) >> ())
    )
);

named!(peculiar_identifier, alt!(tag!("+") | tag!("-") | tag!("...")));

named!(
    identifier<String>,
    map!(
        alt!(peculiar_identifier | common_identifier),
        |s| String::from_utf8_lossy(s).into_owned()
    )
);

// named!(
//     string_content<String>,
//     map!(
//         escaped_transform!(
//             take_until_either!("\"\\"),
//             '\\',
//             alt!(
//                 tag!("\\") => { |_| &b"\\"[..] } |
//                 tag!("\"") => { |_| &b"\""[..] } |
//                 tag!("n") => { |_| &b"\n"[..] } |
//                 tag!("r") => { |_| &b"\r"[..] } |
//                 tag!("t") => { |_| &b"\t"[..] }
//             )
//         ),
//         to_s
//     )
// );

fn parse(line: &str) {
    let res = token(line.as_bytes());
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

macro_rules! assert_parsed_fully {
    ($parser:expr, $input:expr, $result:expr) => {
        assert_eq!($parser($input.as_bytes()), nom::IResult::Done(&b""[..], $result));
    } 
}

#[test]
fn test_boolean() {
    assert_parsed_fully!(boolean, "#t", true);
    assert_parsed_fully!(boolean, "#f", false);
}

#[test]
fn test_character() {
    assert_parsed_fully!(character, "#\\space", ' ');
    assert_parsed_fully!(character, "#\\newline", '\n');
    assert_parsed_fully!(character, "#\\ ", ' ');
    assert_parsed_fully!(character, "#\\X", 'X');
}

#[test]
fn test_integer() {
    assert_parsed_fully!(integer, "1", 1);
    assert_parsed_fully!(integer, "#d+1", 1);
    assert_parsed_fully!(integer, "-1", -1);
    assert_parsed_fully!(integer, "#b010101", 21);
    assert_parsed_fully!(integer, "#o77", 63);
    assert_parsed_fully!(integer, "#xFF", 255);
    assert_parsed_fully!(integer, "#x-ff", -255);
}

#[test]
fn test_token() {
    assert_parsed_fully!(token, "1", Token::Number(1));
    assert_parsed_fully!(token, "else", Token::Keyword(SyntacticKeyword::Else));
    assert_parsed_fully!(token, "lambda", Token::Keyword(
        SyntacticKeyword::Expression(ExpressionKeyword::Lambda))
    );
    assert_parsed_fully!(token, "#\\space", Token::Character(' '));
    // ...
}
