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


impl PasswordPolicy {
    pub fn validate(&self, password: &str) -> bool {
        let counter = password.chars().filter(|c| *c == self.symbol).count() as i32;
        counter >= self.range.min && counter <= self.range.max
    }
}

fn parse(line: &str) -> (PasswordPolicy, &str) {
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

    let password = parts.next().unwrap();

    let policy = PasswordPolicy {
        range,
        symbol,
    };

    return (policy, password);
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
            .map(|line| parse(line))
            .map(|result| result.0.validate(result.1))
            .filter(|valid| *valid == true)
            .count();

        assert_eq!(valid_passwords, 622);
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
    fn should_validate_password_from_single_line() {
        let line = "4-8 n: dnjjrtclnzdnghnbnn";
        let result = parse(line);

        assert_eq!(result.0.validate(result.1), true);
    }
}

