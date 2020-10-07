// pub fn raindrops(n: u32) -> String {
//     let mut result: String = String::from("");
//     let drops = &[(3,"Pling"), (5,"Plang"), (7, "Plong")];
//     for (dropkind,dropsound) in drops {
//         if n % dropkind == 0 {
//             result.push_str(dropsound)
//         }
//     };
//     match result.len() {
//         0 => n.to_string(),
//         _ => result,
//     }
// }
pub fn raindrops(n: u32) -> String {
    let result: String = [(3,"Pling"), (5,"Plang"), (7, "Plong")]
        .iter()
        .filter(|(factor,_)| n % factor == 0)
        .map(|&(_,s)| s)   // tbh i don't fully grasp this ownership/borrow stuff yet :D
        .collect();
    match result.len() {
        0 => n.to_string(),
        _ => result,
    }
}
