// https://cryptohack.org/register/
// wiki: https://en.wikipedia.org/wiki/Caesar_cipher

const SHIFT: u8 = 65;

pub fn caesar_cipher_encryption<const N: usize>(message: &[u8; N], shift: u8) -> String {
    let mut e = [0u8; N];
    let shift = shift % SHIFT;

    for (i, m) in message.iter().enumerate() {
        if *m == b' ' {
            e[i] = *m;
            continue;
        }
        let c = (m - SHIFT + shift) % 26u8;
        e[i] = c + SHIFT;
    }

    e.iter().map(|c| *c as char).collect::<String>()
}

pub fn caesar_cipher_decryption<const N: usize>(cipher_text: &[u8; N], shift: u8) -> String {
    let mut d = [0u8; N];
    let shift = shift % SHIFT;

    for (i, m) in cipher_text.iter().enumerate() {
        if *m == b' ' {
            d[i] = *m;
            continue;
        }
        // Add 26 here for in case overflow
        let c = (26u8 + m - SHIFT - shift) % 26u8;
        d[i] = c + SHIFT;
    }

    d.iter().map(|c| *c as char).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_cipher_encryption_should_work() {
        let message = b"THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
        let shift = 3;
        let cipher_text = "WKH TXLFN EURZQ IRA MXPSV RYHU WKH ODCB GRJ";
        let encrypted = caesar_cipher_encryption::<43>(message, shift);
        assert_eq!(encrypted, cipher_text);
    }

    #[test]
    fn caesar_cipher_decryption_should_work() {
        let message = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
        let shift = 3;
        let cipher_text = b"WKH TXLFN EURZQ IRA MXPSV RYHU WKH ODCB GRJ";
        let decrypted = caesar_cipher_decryption::<43>(cipher_text, shift);
        assert_eq!(decrypted, message);
    }

    #[test]
    fn caesar_cipher_decryption_should_work1() {
        let message = "XEIJ VYISQB LYQRBU FHEFUHJO";
        let shift = 3;
        let cipher_text = b"XEIJ VYISQB LYQRBU FHEFUHJO";

        for i in 0..27u8 {
            let decrypted = caesar_cipher_decryption::<27>(cipher_text, i);
            dbg!(i, decrypted);
        }
    }
}
