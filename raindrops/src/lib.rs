pub fn raindrops(int: i32) -> String {
    match (int % 3, int % 5, int % 7) {
    (0, 0, 0) =>format!("PlingPlangPlong"),
    (0, 0, _) =>format!("PlingPlang"),
    (0, _, 0) =>format!("PlingPlong"),
    (_, 0, 0) =>format!("PlangPlong"),
    (0, _, _) =>format!("Pling"),
    (_, 0, _) =>format!("Plang"),
    (_, _, 0) =>format!("Plong"),
    _ => format!("{}", int)
    }
}
