pub fn verse(num: u8) -> String {
    match num {
        0 => {
            format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and \
             buy some more, 99 bottles of beer on the wall.\n")
        }
        1 => {
            format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it \
                     around, no more bottles of beer on the wall.\n")
        }
        2 => {
            format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it \
                     around, 1 bottle of beer on the wall.\n")
        }
        3...99 => {
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and \
                     pass it around, {less} bottles of beer on the wall.\n",
                    n = num,
                    less = num - 1)
        }
        _ => format!("Something went wrong"),
    }
}

pub fn sing(inital: u8, last: u8) -> String {
    (last..inital).rev().fold(verse(inital), |acc, n| acc + "\n" + &verse(n))
}
