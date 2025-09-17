use nom::IResult;

use crate::error::ParseError;

enum IndentedItem<'a> {
    IndentationMarker(Indentation),
    Block(&'a str),
}

enum Indentation {
    Indent(usize),
    Dedent(usize),
}

pub fn parse_indentation(input: &str) -> Result<Vec<IndentedItem>, ParseError> {
    let mut items = Vec::new();
    let mut current_indent: Indentation = Indentation::Indent(0);

    Ok(items)
}

pub fn same_indentation_block(input: &str) -> IResult<&str, Vec<IndentedItem>> {
    todo!()
}
