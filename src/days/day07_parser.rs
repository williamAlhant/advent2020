use super::day07_struct::{Rule, NumBags, BagTypeRegist};
// use anyhow::{Result, bail};
use nom::{
    // branch::alt,
    bytes::complete::{take_until},
    // character::complete::digit1,
    // multi::separated_list1,
    IResult
};

pub fn parse_rule<'a>(i: &'a str, regist: &mut BagTypeRegist) -> IResult<&'a str, Rule> {

    let mut rule = Rule::default();

    let parser = take_until(" bags");
    let parsed = parser(i)?;

    rule.bag_type = regist.get_or_else_add(parsed.1);

    Ok((parsed.0, rule))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut regist = BagTypeRegist::default();
        let i = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let o = parse_rule(i, &mut regist).unwrap().1;
        assert_eq!(o.to_str(&regist), i);
    }
}