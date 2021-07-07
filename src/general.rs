use num_bigint::BigUint;

/// ENCODING
// ASCII Solution
pub fn ascii() -> String {
	let ords = [
		99u8, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
		108, 51, 125,
	];
	let s = ords.iter().map(|c| *c as char).collect();
	s
}

// Hex Solution
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

// Base64 Solution
// wiki: https://en.wikipedia.org/wiki/Base64
pub fn base64_encode() -> String {
	const BASE64_TABLE: [u8; 64] =
		*b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	let hexed = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
	let s = hexed
		.as_bytes()
		.iter()
		.step_by(2)
		.enumerate()
		.map(|(i, _)| {
			let u = u8::from_str_radix(&hexed[2 * i..(2 * i + 2)], 16).expect("overflow");
			format!("{:08b}", u)
		})
		.collect::<String>();

	s.as_bytes()
		.iter()
		.step_by(6)
		.enumerate()
		.map(|(i, _)| {
			let index = u8::from_str_radix(&s[6 * i..(6 * i + 6)], 2).expect("overflow") as usize;
			BASE64_TABLE[index] as char
		})
		.collect::<String>()
}

// Bytes and Big Integers
pub fn bytes_and_big_integers() -> String {
	let big_int_str =
		*b"11515195063862318899931685488813747395775516287289682636499965282714637259206269";
	let big_int = BigUint::parse_bytes(&big_int_str, 10).expect("Failed to parse big int string");

	let hexed_big_int_hexed = big_int.to_str_radix(16);
	hexed_big_int_hexed
		.as_bytes()
		.iter()
		.step_by(2)
		.enumerate()
		.map(|(i, _)| {
			u8::from_str_radix(&hexed_big_int_hexed[2 * i..(2 * i + 2)], 16).expect("overflow")
				as char
		})
		.collect()
}

// Encoding Challenge
pub fn encoding_challenge() {
	todo!();
}

/// XOR
// XOR Starter
pub fn xor_starter() -> String {
	let target_str = b"label";
	let oprand = 13u8;

	target_str.iter().map(|c| (*c ^ oprand) as char).collect()
}

// XOR Properties
pub fn xor_properties() -> String {
	// KEY1 = a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313
	// KEY2 ^ KEY1 = 37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e
	// KEY2 ^ KEY3 = c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1
	// FLAG ^ KEY1 ^ KEY3 ^ KEY2 = 04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf
	
	let key1 = b"a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
	let key2_key1 = b"37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";
	let key2_key3 = b"c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";
	// flag ^ key1 ^ key3 ^ key2 = 04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf;
	let mut flag = Vec::<u8>::with_capacity(key1.len() / 2);
	todo!()
}

/// MATHEMATICS
// Greatest Common Divisor
pub fn greatest_common_divisor() -> u32 {
	let (mut a, mut b) = (66528u32, 52920u32);
	let d;
	loop {
		let c = a % b;
		if c == 0 {
			d = b;
			break;
		}
		a = b;
		b = c;
	}

	d
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

	#[test]
	fn base64_encode_should_work() {
		let s = base64_encode();
		assert_eq!(s, "crypto/Base+64+Encoding+is+Web+Safe/");
	}

	#[test]
	fn bytes_and_big_integers_should_work() {
		let s = bytes_and_big_integers();
		assert_eq!(s, "crypto{3nc0d1n6_4ll_7h3_w4y_d0wn}");
	}

	#[test]
	fn xor_starter_should_work() {
		let s = xor_starter();
		assert_eq!(s, "aloha");
	}

	#[test]
	fn greatest_common_divisor_should_work() {
		let s = greatest_common_divisor();
		assert_eq!(s, 1512u32);
	}
}
