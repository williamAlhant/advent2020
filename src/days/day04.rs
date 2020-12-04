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
            if passport.byr.is_empty() ||
            passport.iyr.is_empty() ||
            passport.eyr.is_empty() ||
            passport.hgt.is_empty() ||
            passport.hcl.is_empty() ||
            passport.ecl.is_empty() ||
            passport.pid.is_empty() {

            }
            else {
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
}
