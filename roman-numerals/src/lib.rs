use std::fmt::{Display, Formatter, Result};

const ROMAN_MAP: [(usize, &'static str); 13] = [(1, "I"),
                                                 (4, "IV"),
                                                 (5, "V"),
                                                 (9, "IX"),
                                                 (10, "X"),
                                                 (40, "XL"),
                                                 (50, "L"),
                                                 (90, "XC"),
                                                 (100, "C"),
                                                 (400, "CD"),
                                                 (500, "D"),
                                                 (900, "CM"),
                                                 (1000, "M")];

pub struct Roman {
    num: usize,
}

impl From<usize> for Roman {
    fn from(i: usize) -> Self {
        Roman::new(i)
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut start = self.num.clone();
        let mut result = String::new();
        for &(numeric, roman_string) in ROMAN_MAP.into_iter().rev() {
            while start >= numeric {
                result.push_str(roman_string);
                start -= numeric;
            }
        }
        write!(f, "{}", result)
    }
}


impl Roman {
    fn new(num: usize) -> Roman {
        Roman { num: num }
    }
}