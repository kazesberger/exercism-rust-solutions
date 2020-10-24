pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(val) =>
            list.windows(2)
                .map(|it| format!("For want of a {} the {} was lost.\n", it[0], it[1]))
                .chain(std::iter::once(format!("And all for the want of a {}.", val)))
                .collect(),
    }
}

// pub fn build_proverb(list: &[&str]) -> String {
//     let pairs = list.iter().zip(list.iter().skip(1));
//     let all_pairs_rendered = pairs.fold("".to_string(), |a, b| {
//         format!("{}For want of a {} the {} was lost.\n", a, b.0, b.1).to_string()
//     });

//     match list.first() {
//         Some(val) => all_pairs_rendered + &format!("And all for the want of a {}.", val).to_string(),
//         None => "".to_string()
//     }
// }
