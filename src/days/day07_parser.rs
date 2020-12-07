use super::day07_struct::{Rule, NumBags, BagTypeRegist};
use anyhow::{Result, bail};

pub fn parse_rule(i: &str, regist: &mut BagTypeRegist) -> Result<Rule> {
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut regist = BagTypeRegist::default();
        let i = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let o = parse_rule(i, &mut regist).unwrap();
        assert_eq!(o.to_str(&regist), i);
    }
}