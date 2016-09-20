pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, String> {
    if from_base < 2 || to_base < 2 {
        return Err("Wrong base.".to_string());
    } else if number.iter().any(|&num| num >= from_base) {
        return Err("Maybe given number is out of ranges in context to its base.".to_string());
    } else {
        let to_base10 = bin_to_dec(number, from_base);
        let from_base10 = dec_to_bin(to_base10, to_base);
        Ok(from_base10)
    }
}

fn bin_to_dec(number: &[u32], from_base: u32) -> u32 {
    let to_dec = number.iter()
        .rev()
        .enumerate()
        .map(|(power, digit)| digit * from_base.pow(power as u32))
        .fold(0, |sum, n| sum + n);
    to_dec
}

fn dec_to_bin(mut number: u32, to_base: u32) -> Vec<u32> {
    let mut to_bin: Vec<u32> = vec![];
    while number > 0 {
        let num = number % to_base;
        to_bin.push(num);
        number /= to_base;
    }
    to_bin.into_iter().rev().collect()
}
