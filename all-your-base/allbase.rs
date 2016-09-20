pub fn convert(number: &[u64], from_base: u64, to_base: u64) -> Result<Vec<u64>, ()> {
    if from_base < 2 || to_base < 2 || number.iter().any(|&d| d >= from_base) {
        return Err(());
    }

    let mut value = number.iter().fold(0, |acc, &d| acc * from_base + d);
    let mut out = vec![];

    while value > 0 {
        out.push(value % to_base);
        value /= to_base;
    }

    out.reverse();

    Ok(out)
}
