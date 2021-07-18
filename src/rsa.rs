use num_bigint::BigUint;

// RSA Starter 1
pub fn ras_starter_1() -> BigUint {
    let base = BigUint::from(101u32);
    let exponent = BigUint::from(17u32);
    let modulus = BigUint::from(22663u32);
    BigUint::modpow(&base, &exponent, &modulus)
}

// RSA Starter 2
pub fn ras_starter_2() -> BigUint {
    let (p, q) = (17u32, 23u32);
    let n = p * q;
    let e = 65537u32;
    let msg = 12u32;

    BigUint::modpow(&msg.into(), &e.into(), &n.into())
}

// RSA Starter 3
pub fn ras_starter_3() -> BigUint {
    let (p, q) = (
        b"857504083339712752489993810777",
        b"1029224947942998075080348647219",
    );

    let p = BigUint::parse_bytes(p, 10).expect("failed to parse big integer");
    let q = BigUint::parse_bytes(q, 10).expect("failed to parse big integer");
    dbg!(&p * &q);

    (p - 1u32) * (q - 1u32)
}

// RSA Starter 4
pub fn ras_starter_4() -> BigUint {
    let (p, q) = (
        b"857504083339712752489993810777",
        b"1029224947942998075080348647219",
    );

    let p = BigUint::parse_bytes(p, 10).expect("failed to parse big integer");
    let q = BigUint::parse_bytes(q, 10).expect("failed to parse big integer");

    let totient = (p - 1u32) * (q - 1u32);
    let e = 65537u32;
    let mut n = 1u32;
    loop {
        let d = (&totient * n + 1u32) % e;
        // dbg!(n);
        if d == BigUint::from(0u32) {
            return (&totient * n + 1u32) / e;
        }
        n += 1;
    }
}

// RSA Starter 5
pub fn ras_starter_5() -> BigUint {
    let (p, q) = (
        b"857504083339712752489993810777",
        b"1029224947942998075080348647219",
    );

    let p = BigUint::parse_bytes(p, 10).expect("failed to parse big integer");
    let q = BigUint::parse_bytes(q, 10).expect("failed to parse big integer");
    let n = p * q;

    let e = 65537u32;
    // According to the result of RSA Starter 4
    let d = BigUint::parse_bytes(
        b"121832886702415731577073962957377780195510499965398469843281",
        10,
    )
    .expect("failed to parse big integer");

    let c = BigUint::parse_bytes(
        b"77578995801157823671636298847186723593814843845525223303932",
        10,
    )
    .expect("failed to parse big integer");

    BigUint::modpow(&c, &d, &n)
}

// RSA Starter 6
pub fn ras_starter_6() -> BigUint {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ras_starter_1_should_work() {
        let s = ras_starter_1();
        assert_eq!(s, BigUint::from(19906u32));
    }

    #[test]
    fn ras_starter_2_should_work() {
        let s = ras_starter_2();
        assert_eq!(s, BigUint::from(301u32));
    }

    #[test]
    fn ras_starter_3_should_work() {
        let s = ras_starter_3();
        assert_eq!(
            Some(s),
            BigUint::parse_bytes(
                b"882564595536224140639625987657529300394956519977044270821168",
                10
            )
        );
    }

    #[test]
    fn ras_starter_4_should_work() {
        let s = ras_starter_4();
        assert_eq!(
            Some(s),
            BigUint::parse_bytes(
                b"121832886702415731577073962957377780195510499965398469843281",
                10
            )
        );
    }

    #[test]
    fn ras_starter_5_should_work() {
        let s = ras_starter_5();
        assert_eq!(Some(s), BigUint::parse_bytes(b"13371337", 10));
    }
}
