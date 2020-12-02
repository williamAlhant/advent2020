use advent2020::util::input;
use advent2020::util::error_handling::ResultOkPrintErrExt;
use regex::Regex;

#[derive(Debug)]
struct PasswordPolicy {
    a: u32,
    b: u32,
    c: char
}

impl PasswordPolicy {
    pub fn valid_v1(&self, password: &str) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if c == self.c {
                count += 1;
            }
        }
        count >= self.a && count <= self.b
    }

    pub fn valid_v2(&self, password: &str) -> bool {
        let c_at_a = password.as_bytes()[self.a as usize - 1] == self.c as u8;
        let c_at_b = password.as_bytes()[self.b as usize - 1] == self.c as u8;
        (c_at_a as u8 + c_at_b as u8) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_valid_v1() {
        assert!(PasswordPolicy { a: 1, b: 3, c: 'a' }.valid_v1("abcde"));
        assert!(!PasswordPolicy { a: 1, b: 3, c: 'b' }.valid_v1("cdefg"));
        assert!(PasswordPolicy { a: 2, b: 9, c: 'c' }.valid_v1("ccccccccc"));
    }

    #[test]
    fn policy_valid_v2() {
        assert!(PasswordPolicy { a: 1, b: 3, c: 'a' }.valid_v2("abcde"));
        assert!(!PasswordPolicy { a: 1, b: 3, c: 'b' }.valid_v2("cdefg"));
        assert!(!PasswordPolicy { a: 2, b: 9, c: 'c' }.valid_v2("ccccccccc"));
    }
}

fn main() {
    
    let lines = input::lines_from_file_passed_as_argument();

    let ans = do_the_thing(lines);
    if let Some(ans) = ans {
        println!("Answer: {}", ans);
    }
}

// Returns the number of valid passwords
fn do_the_thing<L>(lines: L) -> Option<u32>
    where L: IntoIterator<Item = String> {

    let mut num_valid_passwords = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    for line in lines.into_iter() {
        let captures = re.captures(&line)
            .ok_or_print_err(&format!("No regex match on line \"{}\"", line))?;

        let policy = PasswordPolicy {
            a: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            b: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            c: captures.get(3).unwrap().as_str().parse::<char>().unwrap()
        };

        let password = captures.get(4).unwrap().as_str();

        if policy.valid_v2(password) {
            num_valid_passwords += 1;
        }
    }

    Some(num_valid_passwords)
}
