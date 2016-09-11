pub fn primes_up_to(num: i64) -> Vec<i64> {
    (2..num + 1).fold(vec![], |mut acc, n| {
        if acc.iter().all(|p| (n % p) != 0) {
            acc.push(n);
        }
        acc
    })
}
