use super::day07_struct::{Rule, NumBags, BagTypeRegist};
// use anyhow::{Result, bail};
use nom::{
    IResult, 
    bytes::complete::{take_until, tag}, 
    multi::many1_count,
    character::complete::digit1,
    combinator::opt,
    sequence::preceded
};

pub fn parse_rule<'a>(i: &'a str, regist: &mut BagTypeRegist) -> IResult<&'a str, Rule> {

    let mut rule = Rule::default();

    let (i, bag_type) = take_until(" bags")(i)?;

    rule.bag_type = regist.get_or_else_add(bag_type);

    let (i, _) = preceded(
        tag(" bags contain "), 
        parse_elements(&mut rule, regist)
    )(i)?;

    Ok((i, rule))
}

fn parse_elements<'a>(
    rule: &'a mut Rule,
    regist: &'a mut BagTypeRegist
) -> impl FnMut(&str) -> IResult<&str, ()> + 'a {

    move |i: &str| {
        let (i_remain, _) = many1_count(
            parse_element(rule, regist)
        )(i)?;
        Ok((i_remain,()))
    }
}

fn parse_element<'a>(
    rule: &'a mut Rule,
    regist: &'a mut BagTypeRegist
) -> impl FnMut(&str) -> IResult<&str, ()> + 'a {

    move |j: &str| {

        let mut rule_el = NumBags::default();

        let (remain, digits) = digit1(j)?;
        rule_el.num = digits.parse().unwrap();

        let (remain, bag_type) = preceded(
            tag(" "), 
            take_until(" bag")
        )(remain)?;
        rule_el.bag_type = regist.get_or_else_add(bag_type);

        let (remain, _) = preceded(
            tag(" bag"),
            preceded(
                opt(tag("s")),
                opt(tag(", "))
            )
        )(remain)?;

        rule.elements.push(rule_el);

        Ok((remain, ()))
    }
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