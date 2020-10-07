use std::convert::TryFrom;
use std::ops::RangeFrom;

pub fn nth(n: u32) -> u32 {
    let result = (RangeFrom{start: 2})
                    .filter(|x| (2..=x/2).all(|div| x % div !=0))
                    .nth(usize::try_from(n).unwrap());
    match result {
        Some(prime) => prime,
        None => 0,
    }
}