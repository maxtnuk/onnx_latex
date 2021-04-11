use std::collections::HashMap;

use nom::{combinator::map_res, error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError}, number::complete::{be_u32, be_u8}};
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take, take_until, take_while},
    character::{
        complete::{alphanumeric1 as alphanumeric, char, one_of},
        is_alphabetic, is_alphanumeric,
        streaming::alphanumeric1,
    },
    combinator::{cut, map, opt, value},
    multi::{many0, separated_list1,separated_list0},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    switch, IResult,
};
use nom::number::complete::u8 as u_8;

type Vss<'a> = Vec<(&'a str, &'a str)>;

pub fn symbol_split(input: &str) -> IResult<&str, Vss> {
    many0(tuple((take_while(|ch| !"#$@".contains(ch)), take(3usize))))(input)
}

enum SymbolWhich{
  Input(usize),
  Attribute(usize),
  SelfName(usize)
}

fn from_num(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 10)
}
fn num_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take(1usize),
    from_num
  )(input)
}
pub fn insert_symbol_parts(
    original: (&str, Vss),
    x_in: Vec<String>,
    a_in: Vec<String>,
    s_in: String,
) -> String {
    let mut result = String::new();
    for (key, symbol) in original.1.iter() {
        let parse_result=alt(
          (
            preceded(tag("#_"),map(num_primary,|s:u8| SymbolWhich::Input(s as usize))),
            preceded(tag("@_"),map(num_primary,|s:u8| SymbolWhich::Attribute(s as usize))),
            preceded(tag("$_"),map(num_primary,|s:u8| SymbolWhich::SelfName(s as usize))),
          )
        )(*symbol);

        let to_insert = match parse_result.unwrap().1 {
            SymbolWhich::Input(u ) =>{
              x_in[u].as_str()
            },
            SymbolWhich::Attribute(u) =>{
              a_in[u].as_str()
            },
            SymbolWhich::SelfName(s) => {
              s_in.as_str()
            }
        };
        result += &(key.to_string() + to_insert);
    }
    result += original.0;
    result
}
// parse debug section

#[derive(Debug, PartialEq,Clone)]
pub enum DebugValue {
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<DebugValue>),
    Object(HashMap<String, DebugValue>),
    Tuple(Vec<DebugValue>),
    Undefined(String),
}

impl DebugValue{
  pub fn shallow_to_string(&self) -> String {
      match self{
          DebugValue::Str(ref s) => {s.clone()}
          DebugValue::Boolean(b) => {b.to_string()}
          DebugValue::Num(n) => {n.to_string()}
          DebugValue::Array(a) => {
            let mut result= "[ ".to_string();
            for i in a.iter(){
              result+=&i.shallow_to_string();
              result+=", ";
            }
            result+="]";
            result
          }
          DebugValue::Object(a) => {
            "".to_string()
          }
          DebugValue::Tuple(_) => {
            "".to_string()
          }
          DebugValue::Undefined(s) => {
            s.clone()
          }
      }
  }
}

impl Default for DebugValue{
  fn default() -> Self {
      DebugValue::Undefined("".to_string())
  }
}
/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn sep<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = r#"{(["#;

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| !chars.contains(c))(i)
}

/// A nom parser has the following signature:
/// `Input -> IResult<Input, Output, Error>`, with `IResult` defined as:
/// `type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;`
///
/// most of the times you can ignore the error type and use the default (but this
/// examples shows custom error types later on!)
///
/// Here we use `&str` as input type, but nom parsers can be generic over
/// the input type, and work directly with `&[u8]` or any other type that
/// implements the required traits.
///
/// Finally, we can see here that the input and output type are both `&str`
/// with the same lifetime tag. This means that the produced value is a subslice
/// of the input data. and there is no allocation needed. This is the main idea
/// behind nom's performance.
fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric, '\\', one_of("\"n\\"))(i)
}

/// `tag(string)` generates a parser that recognizes the argument string.
///
/// we can combine it with other functions, like `value` that takes another
/// parser, and if that parser returns without an error, returns a given
/// constant value.
///
/// `alt` is another combinator that tries multiple parsers one by one, until
/// one of them succeeds
fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    // This is a parser that returns `true` if it sees the string "true", and
    // an error otherwise
    let parse_true = value(true, tag("true"));

    // This is a parser that returns `false` if it sees the string "false", and
    // an error otherwise
    let parse_false = value(false, tag("false"));

    // `alt` combines the two parsers. It returns the result of the first
    // successful parser, or an error
    alt((parse_true, parse_false))(input)
}

/// this parser combines the previous `parse_str` parser, that recognizes the
/// interior of a string, with a parse to recognize the double quote character,
/// before the string (using `preceded`) and after the string (using `terminated`).
///
/// `context` and `cut` are related to error management:
/// - `cut` transforms an `Err::Error(e)` in `Err::Failure(e)`, signaling to
/// combinators like  `alt` that they should not try other parsers. We were in the
/// right branch (since we found the `"` character) but encountered an error when
/// parsing the string
/// - `context` lets you add a static string to provide more information in the
/// error chain (to indicate which parser had an error)
fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

/// some combinators, like `separated_list0` or `many0`, will call a parser repeatedly,
/// accumulating results in a `Vec`, until it encounters an error.
/// If you want more control on the parser application, check out the `iterator`
/// combinator (cf `examples/iterator.rs`)
fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<DebugValue>, E> {
    // println!("array {}", i);
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), json_value),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, DebugValue), E> {
    separated_pair(
        preceded(sp, raw_string),
        cut(preceded(sp, char(':'))),
        json_value,
    )(i)
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, HashMap<String, DebugValue>, E> {
    // println!("hash {}", i);
    context(
        "map",
        preceded(
            preceded(
                preceded(take_while(|ch| is_alphanumeric(ch as u8)), take(1usize)),
                char('{'),
            ),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .enumerate()
                            .map(|(i, (k,v))| (i.to_string()+"_"+k, v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(i)
}

fn tuple_it<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<DebugValue>, E> {
    // println!("tupple {}", i);
    context(
        "tupple",
        preceded(
            preceded(take_while(|ch| is_alphanumeric(ch as u8)), char('(')),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), json_value),
                preceded(sp, char(')')),
            )),
        ),
    )(i)
}

fn option_it<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, DebugValue, E> {
    // println!("option {}", i);
    context(
        "option",
        preceded(
            preceded(take_while(|ch: char| is_alphanumeric(ch as u8)), char('(')),
            cut(terminated(json_value, char(')'))),
        ),
    )(i)
}
fn raw_string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("constant", parse_str)(i)
}
/// here, we apply the space parser before trying to parse a value
fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, DebugValue, E> {
    preceded(
        sp,
        alt((
            map(hash, DebugValue::Object),
            map(array, DebugValue::Array),
            map(option_it, |js_value| js_value),
            map(tuple_it, DebugValue::Tuple),
            map(string, |s| DebugValue::Str(String::from(s))),
            map(double, DebugValue::Num),
            map(boolean, DebugValue::Boolean),
            map(raw_string, |s| DebugValue::Undefined(String::from(s))),
        )),
    )(i)
}

/// the root element of a JSON parser is either an object or an array
pub fn op_parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, DebugValue, E> {
    delimited(
        sp,
        alt((
            map(hash, DebugValue::Object),
            map(tuple_it, DebugValue::Tuple),
        )),
        opt(sp),
    )(i)
}

#[test]
fn raw_test() {
    let t = r#"Test { c: [None, Some(2), Some("hello")], b: Hello { a: 20, b: "to" } }"#;
    let result = op_parse::<(&str, ErrorKind)>(t);
    println!("{:?}", result);
    assert!(result.is_ok());
}
#[test]
fn undefined_test() {
    let t = r#"Test(asdfasdf)"#;
    let result = op_parse::<(&str, ErrorKind)>(t);
    println!("{:?}", result);
    assert!(result.is_ok());
}