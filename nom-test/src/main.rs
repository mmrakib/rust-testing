#[allow(unused_imports)]
#[allow(unused_variables)]

mod logo_parsing;
use crate::logo_parsing::{
    observe_parse_whitespace,
    observe_parse_comment,
};

fn main() {
    observe_parse_whitespace();
    observe_parse_comment();  
}
