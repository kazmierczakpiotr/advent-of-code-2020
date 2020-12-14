use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

struct Range {
    min: i32,
    max: i32,
}

struct PasswordPolicy {
    range: Range,
    symbol: char,
}

struct ComplexPasswordPolicy {
    position: Range,
    symbol: char,
}

impl ComplexPasswordPolicy {
    fn from(line: &str) -> ComplexPasswordPolicy {
        let mut parts = line.split_whitespace();
        let range_vec: Vec<i32> = parts.next()
            .expect("Could not fetch next part")
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let position = Range {
            min: *range_vec.get(0).unwrap(),
            max: *range_vec.get(1).unwrap(),
        };

        let symbol = parts.next().unwrap()
            .chars()
            .next()
            .unwrap();

        ComplexPasswordPolicy {
            position,
            symbol,
        }
    }

    pub fn validate(&self, password: &str) -> bool {
        let first_position = password.chars().nth((self.position.min - 1) as usize).unwrap();
        let second_position = password.chars().nth((self.position.max - 1) as usize).unwrap();

        (first_position == self.symbol && second_position != self.symbol)
            || (first_position != self.symbol && second_position == self.symbol)
    }
}

impl PasswordPolicy {
    fn from(line: &str) -> PasswordPolicy {
        let mut parts = line.split_whitespace();
        let range_vec: Vec<i32> = parts.next()
            .expect("Could not fetch next part")
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let range = Range {
            min: *range_vec.get(0).unwrap(),
            max: *range_vec.get(1).unwrap(),
        };

        let symbol = parts.next().unwrap()
            .chars()
            .next()
            .unwrap();

        PasswordPolicy {
            range,
            symbol,
        }
    }

    pub fn validate(&self, password: &str) -> bool {
        let counter = password.chars().filter(|c| *c == self.symbol).count() as i32;
        counter >= self.range.min && counter <= self.range.max
    }
}

fn parse_to_simple_policy(line: &str) -> (PasswordPolicy, &str) {
    let mut parts = line.split(":");
    let value = parts.next().expect("Could not fetch string part");
    let policy = PasswordPolicy::from(value);
    let password = parts.next().unwrap();
    (policy, password)
}

fn parse_to_complex_policy(line: &str) -> (ComplexPasswordPolicy, &str) {
    let mut parts = line.split(":");
    let value = parts.next().expect("Could not fetch string part");
    let policy = ComplexPasswordPolicy::from(value);
    let password = parts.next().unwrap().trim();
    (policy, password)
}

fn read_lines_as_string_vec() -> io::Result<Vec<String>> {
    let mut path_buffer = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buffer.push("resources/day02.txt");
    let input_path = path_buffer.display().to_string();

    BufReader::new(File::open(input_path)?)
        .lines()
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_part() {
        let lines = read_lines_as_string_vec().expect("Could not load file");
        let valid_passwords = lines.iter()
            .map(|line| parse_to_simple_policy(line))
            .map(|result| result.0.validate(result.1))
            .filter(|valid| *valid == true)
            .count();

        assert_eq!(valid_passwords, 622);
    }

    #[test]
    fn should_solve_second_part() {
        let lines = read_lines_as_string_vec().expect("Could not load file");
        let valid_passwords = lines.iter()
            .map(|line| parse_to_complex_policy(line))
            .map(|result| result.0.validate(result.1))
            .filter(|valid| *valid == true)
            .count();

        assert_eq!(valid_passwords, 263);
    }

    #[test]
    fn should_validate_password_against_policy() {

        // "4-8 n: dnjjrtclnzdnghnbnn";
        let mut range = Range { min: 4, max: 8 };
        let mut policy = PasswordPolicy {
            range,
            symbol: 'n',
        };
        assert_eq!(policy.validate("dnjjrtclnzdnghnbnn"), true);


        // line = "13-14 k: kkkkkkkkkkkkk";
        range = Range { min: 13, max: 18 };
        policy = PasswordPolicy {
            range,
            symbol: 'k',
        };

        assert_eq!(policy.validate("kkkkkkkkkkkkk"), true);
    }

    #[test]
    fn should_validate_password_against_complex_policy() {

        // "1-3 a: abcde";
        let mut range = Range { min: 1, max: 3 };
        let mut policy = ComplexPasswordPolicy {
            position: range,
            symbol: 'a',
        };
        assert_eq!(policy.validate("abcde"), true);

        // "1-3 a: dbade";
        let mut range = Range { min: 1, max: 3 };
        let mut policy = ComplexPasswordPolicy {
            position: range,
            symbol: 'a',
        };
        assert_eq!(policy.validate("dbade"), true);

        // line = "1-3 b: cdefg";
        range = Range { min: 1, max: 3 };
        policy = ComplexPasswordPolicy {
            position: range,
            symbol: 'b',
        };

        assert_eq!(policy.validate("cdefg"), false);

        // line = "2-9 c: ccccccccc";
        range = Range { min: 2, max: 9 };
        policy = ComplexPasswordPolicy {
            position: range,
            symbol: 'c',
        };

        assert_eq!(policy.validate("ccccccccc"), false);

        // line = "2-4 v: jvgxmphvr";
        range = Range { min: 2, max: 4 };
        policy = ComplexPasswordPolicy {
            position: range,
            symbol: 'v',
        };

        assert_eq!(policy.validate("jvgxmphvr"), true);
    }

    #[test]
    fn should_validate_password_from_single_line() {
        let line = "4-8 n: dnjjrtclnzdnghnbnn";
        let result = parse_to_simple_policy(line);

        assert_eq!(result.0.validate(result.1), true);
    }
}

