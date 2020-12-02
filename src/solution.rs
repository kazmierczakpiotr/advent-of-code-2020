pub fn execute(arr: &mut [i32]) -> i32 {
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

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let mut x: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(execute(&mut x), 514579);
    }
}
