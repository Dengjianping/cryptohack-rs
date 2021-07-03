/// ENCODING
// ASCII
pub fn ascii() -> String {
    let ords = [
        99u8, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
        108, 51, 125,
    ];
    let s = ords.iter().map(|c| *c as char).collect();
    s
}

// Hex
pub fn hex() -> String {
    let hexed = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    hexed
        .as_bytes()
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(i, _)| {
            u8::from_str_radix(&hexed[2 * i..(2 * i + 2)], 16)
                .map(|c| c as char)
                .expect("overflow")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_should_work() {
        let s = ascii();
        assert_eq!(s, "crypto{ASCII_pr1nt4bl3}");
    }

    #[test]
    fn hex_should_work() {
        let s = hex();
        assert_eq!(s, "crypto{You_will_be_working_with_hex_strings_a_lot}");
    }
}
