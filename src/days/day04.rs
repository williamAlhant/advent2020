use advent2020::util::input;
use anyhow::{Context, Result, anyhow, bail};

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument();

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing<L>(lines: L) -> Result<u64>
where L: IntoIterator<Item = String> {
    
    let mut num_valid = 0;
    let mut paragraph = String::new();

    for line in lines.into_iter() {
        if line.is_empty() {
            let passport = PassportBeforeValidation::from_paragraph(&paragraph);
            if valid_passport(&passport) {
                num_valid += 1;
            }

            paragraph.clear();
            continue;
        }
        paragraph.push_str(&line);
        paragraph.push('\n');
    }

    Ok(num_valid)
}

#[derive(PartialEq)]
struct StringPairRef<'a> {
    a: &'a str,
    b: &'a str
}

struct StringPairRefIterator<'a> {
    input: &'a str,
    pos: usize
}

impl<'a> StringPairRefIterator<'a> {
    fn from_input(input: &'a str) -> Self {
        Self {
            input,
            pos: 0
        }
    }
}

impl<'a> Iterator for StringPairRefIterator<'a> {
    type Item = StringPairRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.pos >= self.input.len() {
            return None;
        }

        let pos_a = self.pos;
        let mut pos_b = self.pos;
        let mut chars = self.input[self.pos..].chars();
        while let Some(c) = chars.next() {
            match c {
                ':' => pos_b = self.pos + 1,
                ' '|'\n' => break,
                _ => ()
            }
            self.pos += 1;
        }
        let res = Some(StringPairRef {
            a: &self.input[pos_a..pos_b - 1],
            b: &self.input[pos_b..self.pos]
        });
        self.pos += 1;
        res
    }
}

#[derive(Default)]
struct PassportBeforeValidation {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl PassportBeforeValidation {
    fn from_paragraph(paragraph: &str) -> Self {
        let mut p = Self::default();
        for string_pair in StringPairRefIterator::from_input(paragraph) {
            match string_pair.a {
                "byr" => p.byr.push_str(string_pair.b),
                "iyr" => p.iyr.push_str(string_pair.b),
                "eyr" => p.eyr.push_str(string_pair.b),
                "hgt" => p.hgt.push_str(string_pair.b),
                "hcl" => p.hcl.push_str(string_pair.b),
                "ecl" => p.ecl.push_str(string_pair.b),
                "pid" => p.pid.push_str(string_pair.b),
                "cid" => p.cid.push_str(string_pair.b),
                _ => ()
            }
        }
        p
    }
}

struct ValidatedPassport {

}

impl ValidatedPassport {
    fn from_unvalidated(p: &PassportBeforeValidation) -> Result<ValidatedPassport> {

        let byr = p.byr.parse::<u32>()?;
        if !(1920..=2002).contains(&byr) {
            bail!("");
        }

        let iyr = p.iyr.parse::<u32>()?;
        if !(2010..=2020).contains(&iyr) {
            bail!("");
        }

        let eyr = p.eyr.parse::<u32>()?;
        if !(2020..=2030).contains(&eyr) {
            bail!("");
        }

        if p.hgt.len() < 2 {
            bail!("");
        }
        let hgt = p.hgt[0..(p.hgt.len() - 2)].parse::<u32>()?;
        match &p.hgt[(p.hgt.len() - 2)..] {
            "cm" => {
                if !(150..=193).contains(&hgt) {
                    bail!("");
                }
            },
            "in" => {
                if !(59..=76).contains(&hgt) {
                    bail!("");
                }
            },
            _ => bail!("")
        }

        if !p.hcl.starts_with("#") {
            bail!("");
        }
        if !p.hcl[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            bail!("");
        }

        let allowed_ecl_values = 
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !allowed_ecl_values.contains(&p.ecl.as_str()) {
            bail!("");
        }

        if p.pid.chars().count() != 9 || !p.pid.chars().all(|c| c.is_ascii_digit()) {
            bail!("");
        }

        Ok(ValidatedPassport {})
    }
}

fn valid_passport(p: &PassportBeforeValidation) -> bool {
   let p = ValidatedPassport::from_unvalidated(p);
   p.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_iterator() {
        let input = "a:b co:#be";
        let mut it = StringPairRefIterator::from_input(input);
        let expected1 = StringPairRef { a: "a", b: "b" };
        let expected2 = StringPairRef { a: "co", b: "#be" };
        assert!(it.next().unwrap() == expected1);
        assert!(it.next().unwrap() == expected2);
        assert!(it.next().is_none());
    }

    #[test]
    fn passport_from_paragraph() {
        let paragraph = 
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
            byr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport = PassportBeforeValidation::from_paragraph(paragraph);
        assert!(passport.ecl == "gry");
        assert!(passport.pid == "860033327");
        assert!(passport.eyr == "2020");
        assert!(passport.hcl == "#fffffd");
        assert!(passport.byr == "1937");
        assert!(passport.iyr == "2017");
        assert!(passport.cid == "147");
        assert!(passport.hgt == "183cm");
    }

    #[test]
    fn valid_passports() {
        assert!(valid_passport(&PassportBeforeValidation {
            pid:"087499704".to_string(), hgt:"74in".to_string(), ecl:"grn".to_string(), 
            iyr:"2012".to_string(), eyr:"2030".to_string(), byr:"1980".to_string(), hcl:"#623a2f".to_string(),
            ..Default::default()
        }));

        assert!(valid_passport(&PassportBeforeValidation {
            eyr:"2029".to_string(), ecl:"blu".to_string(), cid:"129".to_string(), byr:"1989".to_string(),
            iyr:"2014".to_string(), pid:"896056539".to_string(), hcl:"#a97842".to_string(), hgt:"165cm".to_string(),
            ..Default::default()
        }));

        assert!(valid_passport(&PassportBeforeValidation {
            hcl:"#888785".to_string(),
            hgt:"164cm".to_string(), byr:"2001".to_string(), iyr:"2015".to_string(), cid:"88".to_string(),
            pid:"545766238".to_string(), ecl:"hzl".to_string(),
            eyr:"2022".to_string(),
            ..Default::default()
        }));

        assert!(valid_passport(&PassportBeforeValidation {
            iyr:"2010".to_string(), hgt:"158cm".to_string(), hcl:"#b6652a".to_string(), ecl:"blu".to_string(), 
            byr:"1944".to_string(), eyr:"2021".to_string(), pid:"093154719".to_string(),
            ..Default::default()
        }));
    }

    #[test]
    fn invalid_passports() {
        assert!(!valid_passport(&PassportBeforeValidation {
            eyr:"1972".to_string(), cid:"100".to_string(), hcl:"#18171d".to_string(), 
            ecl:"amb".to_string(), hgt:"170".to_string(), pid:"186cm".to_string(), iyr:"2018".to_string(),
            byr:"1926".to_string(),
            ..Default::default()
        }));
    }
}
