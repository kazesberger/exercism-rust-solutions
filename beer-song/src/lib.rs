const SPECIAL_VERSES: [&'static str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
];

pub fn verse(n: u32) -> String {
    match n {
        0..=2 => SPECIAL_VERSES[n as usize].to_string(),
        3..=99 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, (n-1)),
        _ => panic!()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).into_iter().rev().map(verse).collect::<Vec<String>>().join("\n")
}
