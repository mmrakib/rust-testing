#[allow(unused_imports)]
#[allow(unused_variables)]

use nom::{
    branch::alt, bytes::complete::{is_not, tag, tag_no_case, take_while1},
    character::complete::{alpha1, alphanumeric1, digit1, multispace0, multispace1, one_of, line_ending, not_line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Err, IResult, UnspecializedInput,
};

type ParserError<'a> = nom::error::VerboseError<&'a str>;

fn parse_whitespace(input: &str) -> IResult<&str, &str, ParserError> {
    map(multispace0, |x| x)(input)
}

fn parse_comment(input: &str) -> IResult<&str, (), ParserError> {
    map(
        preceded(
            tag("//"),
            terminated(not_line_ending, line_ending)
        ),
        |_| ()
    )(input)
}

pub fn observe_parse_whitespace() {
    let input: &str = "          \n";

    match parse_whitespace(input) {
        Ok(output) => {
            println!("{:?}", output);
        },
        Err(error) => {
            panic!("{:?}", error);
        }
    };
}

pub fn observe_parse_comment() {
    let input: &str = "// Hello, this is a comment.\n";

    match parse_comment(input) {
        Ok(output) => {
            println!("{:?}", output);
        },
        Err(error) => {
            panic!("{:?}", error);
        }
    }
}
