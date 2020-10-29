pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut rest_num = n;

    let mut factor_candidates = {2..=n}
                                .filter(|x| n % x == 0);

    while rest_num > 1 {
        let factor_candidate = factor_candidates.next().unwrap();
        while rest_num % factor_candidate == 0 {
            result.push(factor_candidate);
            rest_num /= factor_candidate
        }
    }

    result
}

// pub fn factors(n: u64) -> Vec<u64> {
//     let mut result = Vec::new();

//     match (2..n+1).find(|x| n%x == 0) {
//         Some(x) => {
//             result.push(x);
//             result.append(&mut factors(n/x));
//         },
//         None => {}
//     }
//     return result
// }