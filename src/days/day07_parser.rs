use super::day07_struct::{Rule, BagTypeRegist};
use anyhow::{Result, bail};

pub fn parse_rule(i: &str) -> Result<Rule> {
    bail!("");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let regist = BagTypeRegist::default();
        let i = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let o = parse_rule(i).unwrap();
        assert!(o.to_str(&regist) == i);
    }
}