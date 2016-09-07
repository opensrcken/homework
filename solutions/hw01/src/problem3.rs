use std::collections::HashSet;

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut seen: HashSet<u32> = HashSet::new();
    let mut primes: Vec<u32> = Vec::new();

    for i in 2..n {
        if seen.contains(&i) {
            continue;
        }

        primes.push(i);

        let mut multiple = i;
        loop {
            let current = i * multiple;
            if current > n {
                break;
            }
            seen.insert(current);
            multiple += 1;
        }
    }

    primes
}