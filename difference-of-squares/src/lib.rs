pub fn square_of_sum(num: usize) -> usize {
    (1..num + 1).fold(0, |sum, next| sum + next).pow(2)
}
pub fn sum_of_squares(num: usize) -> usize {
    (1..num + 1).fold(0, |sum, next| sum + (next.pow(2)))
}

pub fn difference(num: usize) -> usize {
    square_of_sum(num) - sum_of_squares(num)
}
