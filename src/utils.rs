// General solution for Great Snakes.
pub fn general_great_snake(message: &[u8]) -> String {
    let prefix = vec![81u8, 64, 75, 66, 70, 93, 73];
    let post = vec![79u8];
    let a = vec![prefix, message.to_vec(), post];
    let s = a
        .into_iter()
        .flatten()
        .map(|c| (c ^ 0x32) as char)
        .collect::<String>();
    println!("Here is your flag: {}", s);

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn general_great_snake_should_work() {
        let cipher_text = b"pants";
        let s = general_great_snake(cipher_text);
        assert_eq!(s, "");
    }
}
