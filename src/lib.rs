use std::ops::RangeFrom;

use nom::{
    bytes::complete::take, combinator::fail, IResult, InputIter, InputLength, InputTake, Parser,
    Slice,
};

/// Greedily collect matches for the first parser, discarding anything that does not match until the second parser succeeds
/// Returns Err if the second parser does not succeed before EOF
pub fn find_many_till<I, O, P>(
    mut f: impl FnMut(I) -> IResult<I, O>,
    mut g: impl FnMut(I) -> IResult<I, P>,
    inp: I,
) -> IResult<I, (Vec<O>, P)>
where
    I: Clone + InputTake + InputLength + InputIter,
{
    let mut all_found = Vec::new();
    let mut inp = inp;
    loop {
        match g(inp.clone()) {
            Err(_) => {
                if let Ok((rest, found)) = f(inp.clone()) {
                    all_found.push(found);
                    inp = rest;
                } else {
                    let (rest, _) = take(1usize)(inp)?;
                    inp = rest;
                }
            }
            Ok((rest, second_parser_match)) => return Ok((rest, (all_found, second_parser_match))),
        }
    }
}

/// finds all non-overlapping matches for a given parser until the input is exhausted
pub fn find_all<I, O, E, F>(mut pat: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: InputIter + InputTake + Clone + InputLength + Slice<RangeFrom<usize>>,
    F: Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    move |mut i: I| {
        let mut all_found = Vec::new();
        loop {
            match pat.parse(i.clone()) {
                Ok((rest, found)) => {
                    i = rest;
                    all_found.push(found);
                }
                Err(_) => {
                    if i.input_len() > 0 {
                        i = i.slice(1..);
                    } else {
                        return Ok((i, all_found));
                    }
                }
            }
        }
    }
}

/// Eat input until the parser succeeds
pub fn find_next<I, O, F>(mut f: F) -> impl FnMut(I) -> IResult<I, O>
where
    F: FnMut(I) -> IResult<I, O>,
    I: Clone + InputLength + Slice<RangeFrom<usize>>,
{
    move |mut inp: I| {
        while inp.input_len() > 0 {
            if let Ok(found) = f.parse(inp.clone()) {
                return Ok(found);
            } else {
                inp = inp.slice(1..);
            }
        }
        fail(inp)
    }
}
