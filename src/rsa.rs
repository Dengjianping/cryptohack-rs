use num_bigint::BigUint;
use sha2::{Digest, Sha256};

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
pub fn ras_starter_6() -> String {
    let n = b"152165836548367313276399812241339188558959483740723840508484799\
    0898228689073176948660908591885766404607537525316895505874318566439027305\
    8074450390236774324903305663479046566232967297765731625328029814055635316\
    0025912275702712714452260949198644754078844599804896380010927885748115541\
    4977402895031069511268872385376374323875334978250812198533874675523781937\
    3178699343135091783992299561827389745132880022259873387524273298850340648\
    7798979093819797140268371720039532210524312179406325529308800009194365072\
    4515072654304071472155336106331195428528985758207988029519963275782952572\
    3874753306371990452491305564061051059885803";

    let d = b"111759012106430142625482224734495330913788482694905188504743996\
    8169054728166505931715583169230045319733573572845925939236682330240568538\
    9586883670043744683993709123180805154631088513521456979317628012721881537\
    1541072393894660631360073371205999154566597585593006734446892638549213321\
    8556270670757366065816499109845787449505485449147406503962192297267158829\
    9315846306069845169959451250821044417886630346229021305410340100401530146\
    1354188065443409083551065820890829805336510955941920314116798661342564182\
    9224959213544114538446626127942879540872199056465870390378795695816844984\
    1491667690491585550160457893350536334242689";

    let n = BigUint::parse_bytes(n, 10).expect("failed to parse big integer");
    let d = BigUint::parse_bytes(d, 10).expect("failed to parse big integer");

    let msg = b"crypto{Immut4ble_m3ssag1ng}";

    // Hash message
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    let hexed_msg = hex::encode(result);

    let m = BigUint::parse_bytes(&hexed_msg.as_bytes(), 16).expect("failed to parse hex string.");

    // sign message
    let signed = BigUint::modpow(&m, &d, &n);
    format!("{:0x}", signed)
}

// Primes Part 1
// Factoring
pub fn factoring() -> BigUint {
    let n = b"510143758735509025530880200653196460532653147";
    let n = BigUint::parse_bytes(n, 10).expect("failed to parse big integer");

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

    #[test]
    fn ras_starter_6_should_work() {
        let s = ras_starter_6();
        assert_eq!(
            s,
            "6ac9bb8f110b318a40ad8d7e57defdcce2652f5928b5f9b97c1504d7096d7af1d34\
            e477b30f1a08014e8d525b14458b709a77a5fa67d4711bd19da1446f9fb0ffd9fded\
            c4101bdc9a4b26dd036f11d02f6b56f4926170c643f302d59c4fe8ea678b3ca91b4b\
            b9b2024f2a839bec1514c0242b57e1f5e77999ee67c450982730252bc2c3c35acb4a\
            c06a6ce8b9dbf84e29df0baa7369e0fd26f6dfcfb22a464e05c5b72baba8f78dc742\
            e96542169710918ee2947749477869cb3567180ccbdfe6fdbe85bcaca4bf6da77c8f\
            382bb4c8cd56dee43d1290ca856318c97f1756b789e3cac0c9738f5e9f797314d39a\
            2ededb92583d97124ec6b313c4ea3464037d3"
        );
    }

    #[test]
    #[ignore]
    fn factoring_should_work() {
        let s = factoring();
    }
}
