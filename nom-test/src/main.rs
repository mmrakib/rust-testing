use nom::{
    IResult,
    character::complete::{alpha1, space0},
};

fn parse_command_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_space(input: &str) -> IResult<&str, &str> {
    space0(input)
}

fn main() {
    let input = "abc def";
    let result = parse_command_name(input).unwrap();
    let result2 = parse_space(result.0).unwrap();
    let result3 = parse_command_name(result2.0).unwrap();

    println!("Result 1: {:?}", result);
    println!("Result 2: {:?}", result3);
}
