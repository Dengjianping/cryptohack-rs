// Finding Flags.
// This is too easy, just skip it.

// Solution for Great Snakes.
pub fn great_snake() -> String {
    let ords = [
        81u8, 64, 75, 66, 70, 93, 73, 72, 1, 92, 109, 2, 84, 109, 66, 75, 70, 90, 2, 92, 79,
    ];
    let s = ords.iter().map(|c| (*c ^ 0x32) as char).collect::<String>();
    println!("Here is your flag: {}", s);

    s
}

// Solution for Network Attacks.
pub fn network_attacks() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn great_snake_should_work() {
        let cipher_text = "crypto{z3n_0f_pyth0n}";
        let s = great_snake();
        assert_eq!(s, cipher_text);
    }
}
