use super::day16_struct::Field;
use anyhow::Result;

use nom::{
    IResult, 
    bytes::complete::{take_until, tag}, 
    character::complete::digit1,
    sequence::tuple,
    combinator::map_res
};
use std::ops::RangeInclusive;

pub fn parse_field<'a>(i: &'a str) -> IResult<&'a str, Field> {

    let (i, field_name) = take_until(":")(i)?;

    let (i, _) = tag(": ")(i)?;

    let (i, (range1, _, range2)) = tuple((parse_range, tag(" or "), parse_range))(i)?;

    let field = Field {
        name: field_name.to_string(),
        range1,
        range2
    };

    Ok((i, field))
}

pub fn parse_range<'a>(i: &'a str) -> IResult<&'a str, RangeInclusive<u32>> {

    match tuple((parse_u32, tag("-"), parse_u32))(i) {
        Ok((i, (a, _, b))) => Ok((i, a..=b)),
        Err(e) => Err(e)
    }
}

fn parse_u32(input: &str
) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let input = "departure location: 27-672 or 680-954";
        let expected = Field {
            name: "departure location".to_string(),
            range1: 27..=672,
            range2: 680..=954
        };
        assert_eq!(parse_field(input)?.1, expected);
        Ok(())
    }
}