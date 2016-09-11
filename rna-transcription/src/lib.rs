#[derive(Debug, PartialEq, Eq)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(rna: &str) -> RibonucleicAcid {
        RibonucleicAcid(rna.to_owned())
    }
}

pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(dna.to_owned())
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid(self.0.chars().map(|dna| interchanged(dna)).collect())
    }
}

fn interchanged(ch: char) -> char {
    match ch {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => 'X',
    }
}
