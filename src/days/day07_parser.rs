use super::day07_struct::{Rule, NumBags, BagTypeRegist};
// use anyhow::{Result, bail};
use nom::{
    IResult, 
    bytes::complete::{take_until, tag}, 
    multi::many1_count,
    character::complete::digit1,
    combinator::{opt, map},
    branch::alt,
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
        let (i_remain, _) = alt((
            map(tag("no other bags"), |_| 0),
            many1_count(
                parse_element(rule, regist)
            )
        ))(i)?;

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

    fn parse_and_back_to_str(s: &str, regist: &mut BagTypeRegist) -> String {
        parse_rule(s, regist).unwrap().1.to_str(regist)
    }

    #[test]
    fn parse() {
        let mut regist = BagTypeRegist::default();
        let i = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "bright white bags contain 1 shiny gold bag.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
        let i = "faded blue bags contain no other bags.";
        assert_eq!(parse_and_back_to_str(i, &mut regist), i);
    }
}