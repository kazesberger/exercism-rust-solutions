use std::convert::TryInto;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut rest_num = num;
    let mut digits = vec![];
    while rest_num > 0 {
        digits.push(rest_num % 10);
        rest_num /= 10;
    }
    num == digits.iter().fold(0, |acc, x| acc + u32::pow(*x, digits.len().try_into().unwrap()))
}
