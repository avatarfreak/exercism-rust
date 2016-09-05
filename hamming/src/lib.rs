pub fn hamming_distance(dna: &str, dna2: &str) -> Result<usize, String> {
    let pattern_test = dna.chars().zip(dna2.chars()).filter(|&(d1, d2)| d1 != d2).count();
    if dna.len() == dna2.len() {
        Ok(pattern_test)
    }else{
        Err(String::from("Difference in length"))
    }
}
