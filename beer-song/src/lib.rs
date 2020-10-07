fn verse_params(n: u32) -> (String, String, String, String, String) {
    match n {
        2 => ("2".to_owned(), "bottles".to_owned(), "Take one down and pass it around".to_owned(), "1".to_owned(), "bottle".to_owned()),
        1 => ("1".to_owned(), "bottle".to_owned(), "Take it down and pass it around".to_owned(), "no more".to_owned(), "bottles".to_owned()),
        0 => ("No more".to_owned(), "bottles".to_owned(), "Go to the store and buy some more".to_owned(), "99".to_owned(), "bottles".to_owned()),
        3..=99 => (n.to_string(), "bottles".to_owned(), "Take one down and pass it around".to_owned(), (n-1).to_string().to_owned(), "bottles".to_owned()),
        _ => panic!()
    }
}
pub fn verse(n: u32) -> String {
    let (starting_beers, bottlez_start, action, resulting_beers, bottlez_result) = verse_params(n);
    format!("{} {} of beer on the wall, {} {} of beer.\n{}, {} {} of beer on the wall.\n", starting_beers, bottlez_start, starting_beers.to_lowercase(), bottlez_start, action, resulting_beers, bottlez_result)
}

pub fn sing(start: u32, end: u32) -> String {
    let v: Vec<String> = (end..=start).into_iter().rev().map(verse).collect();
    v.join("\n")
}
