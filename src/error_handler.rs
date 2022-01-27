use errors;
use inner_errors::ParserError;
use std::path::Path;
use types::{Input, PrintableByteSlice};

pub fn map_err(input: &[u8], err: ::nom::Err<Input, ParserError>) -> errors::Error {
    use nom::Err;

    match err {
        Err::Incomplete(_) => unreachable!(),

        Err::Failure(ref context) | Err::Error(ref context) => {
            let errors = ::nom::error_to_list(context);
            {
                let printable_errors: Vec<_> = errors.iter().map(|(_leftover, err)| err).collect();
                println!("{:#?}", printable_errors);
            }
            let relevant_errors = custom_parser_errors(&errors);
            match &relevant_errors[..] {
                [.., (leftover, err)] => map_single_error(input, leftover, **err),
                _ => errors::Error::ParsingError {
                    err: ParserError::FromNom,
                    line: 0,
                },
            }
        }
    }
}

pub fn map_single_error(input: &[u8], leftover: &Input, err: ParserError) -> errors::Error {
    let offset = input.len() - leftover.0.len();
    let line = get_line_number(input, offset);

    errors::Error::ParsingError { err, line }
}

fn custom_parser_errors<'a>(
    errors: &'a Vec<(Input, ::nom::ErrorKind<ParserError>)>,
) -> Vec<(&'a Input<'a>, &'a ParserError)> {
    errors
        .iter()
        .filter_map(|(leftover, kind)| match kind {
            ::nom::ErrorKind::Custom(err) => Some((leftover, err)),
            _ => None,
        })
        .collect()
}

fn split_off_nom_errors<'a>(
    errors: &'a Vec<(Input, ::nom::ErrorKind<ParserError>)>,
) -> &'a [(Input<'a>, ::nom::ErrorKind<ParserError>)] {
    let pos = errors.iter().rposition(|(_leftover, kind)| match kind {
        ::nom::ErrorKind::Custom(_) => true,
        _ => false,
    });

    match pos {
        None => return &errors[0..0],
        Some(pos) => return &errors[0..=pos],
    }
}

pub fn last_parser_error<I>(
    errors: Vec<(I, ::nom::ErrorKind<ParserError>)>,
) -> Option<(I, ParserError)> {
    errors
        .into_iter()
        .filter_map(|(leftover, kind)| match kind {
            ::nom::ErrorKind::Custom(err) => Some((leftover, err)),
            _ => None,
        })
        .last()
}

fn print_leftover(leftover: &[u8]) {
    println!("{:#?}", PrintableByteSlice(leftover));
}

pub fn handle_single_error<P: AsRef<Path>>(
    path: P,
    input: &[u8],
    leftover: &Input,
    err: &::nom::ErrorKind<ParserError>,
) {
    let offset = input.len() - leftover.0.len();
    let description = match err {
        ::nom::ErrorKind::Custom(custom) => custom.description(),
        otherwise => otherwise.description(),
    };

    println!(
        "Error on line {} in file {:#?}: {:#?}",
        get_line_number(input, offset),
        path.as_ref(),
        description
    );
}

fn get_line_number(content: &[u8], offset: usize) -> usize {
    content[0..offset].iter().filter(|b| **b == b'\n').count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let input = b"a\nb\nc\nd";
        let expected = 4;

        let actual = get_line_number(input, 7);

        assert_eq!(expected, actual);
    }
}
