pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for num in 2..=upper_bound {
        let mut append = true;
        for element in result.iter() {
            if num % element == 0 {
                append = false;
            }
        }
        if append == true {
            result.push(num);
        }
    }
    result
}
