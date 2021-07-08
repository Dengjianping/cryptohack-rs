use num_bigint::BigUint;

// RSA Starter 1
pub fn ras_starter_1() -> BigUint {
    let base = BigUint::from(101u32);
    let exponent = BigUint::from(17u32);
    let modulus = BigUint::from(22663u32);
    BigUint::modpow(&base, &exponent, &modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ras_starter_1_should_work() {
        let s = ras_starter_1();
        assert_eq!(s, BigUint::from(19906u32));
    }
}
