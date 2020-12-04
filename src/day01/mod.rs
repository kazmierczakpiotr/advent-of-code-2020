use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

fn solve_basic_data_set(arr: &mut [i32]) -> i32 {
    let mut result = 1;
    for el in arr.iter() {
        for nested_el in arr.iter() {
            let sum = el + nested_el;
            if sum == 2020 {
                print!("{} and s: {} sums up to: {} \n", el.to_string(), nested_el.to_string(), sum);
                result = multiply(*el, *nested_el);
            }
        }
    }
    result
}

fn solve_main_data_set() -> i32 {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/day01.txt");
    let input_path = d.display().to_string();

    let lines = read_lines(input_path)
        .expect("Could not load file");

    let numbers: Vec<i32> = lines.iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut result = 1;
    for el in numbers.iter() {
        for nested_el in numbers.iter() {
            let sum = el + nested_el;
            if sum == 2020 {
                print!("{} and s: {} sums up to: {} \n", el.to_string(), nested_el.to_string(), sum);
                result = multiply(*el, *nested_el);
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_basic_example() {
        let mut x: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_basic_data_set(&mut x), 514579);
    }
}
