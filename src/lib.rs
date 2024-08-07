use anyhow::Result;
use errors::DecodeError;

pub mod errors;

pub struct Base16 {}

impl Base16 {
    /// Base16 encoding.
    ///
    /// # Example
    /// ```
    /// use simple_encode:Base16;
    ///
    /// fn main() {
    ///     let input = b"Test";
    ///     let enc = Base16::encode(input).unwarp();
    ///     println!("{}", enc);
    /// }
    /// ```
    pub fn encode(input: &[u8]) -> Result<String> {
        let encoded = input.iter().map(|byte| format!("{:02x}", byte)).collect();
        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        if input.len() % 2 != 0 {
            Err(DecodeError::new("hex string has an odd length").into())
        } else {
            let mut ret = Vec::new();
            for i in (0..input.len()).step_by(2) {
                let value = u8::from_str_radix(&input[i..i + 2], 16)?;
                // println!("{} - {}", &input[i..i + 2], value);
                ret.push(value);
            }
            Ok(ret)
        }
    }
}

const BASE32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub struct Base32 {}

impl Base32 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut encoded = String::new();
        let mut buffer = 0u32;
        let mut bits_left = 0;

        for &byte in input {
            buffer = (buffer << 8) | (byte as u32);
            bits_left += 8;

            while bits_left >= 5 {
                bits_left -= 5;
                let index = (buffer >> bits_left) & 0b11111;
                let value = match BASE32_ALPHABET.chars().nth(index as usize) {
                    Some(v) => v,
                    None => {
                        return Err(DecodeError::new(&format!(
                            "get value from alphabet failed, index {}, max length {}",
                            index,
                            BASE32_ALPHABET.len(),
                        ))
                        .into())
                    }
                };
                encoded.push(value);
            }
        }

        if bits_left > 0 {
            let index = (buffer << (5 - bits_left)) & 0b11111;
            let value = match BASE32_ALPHABET.chars().nth(index as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        index,
                        BASE32_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
        }

        while encoded.len() % 8 != 0 {
            encoded.push('=');
        }

        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut decoded = Vec::new();
        let mut buffer = 0u32;
        let mut bits_left = 0;

        for c in input.chars() {
            if c == '=' {
                break;
            }

            let index = match BASE32_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base32 character").into()),
            };
            buffer = (buffer << 5) | (index as u32);
            bits_left += 5;

            if bits_left >= 8 {
                bits_left -= 8;
                decoded.push((buffer >> bits_left) as u8);
            }
        }

        Ok(decoded)
    }
}

const BASE36_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

pub struct Base36 {}

impl Base36 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut num = 0u128;
        for byte in input {
            num = (num << 8) | *byte as u128;
        }

        let mut encoded = String::new();
        while num > 0 {
            let remainder = num % 36;
            num /= 36;
            let value = match BASE36_ALPHABET.chars().nth(remainder as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        remainder,
                        BASE36_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
        }

        if encoded.is_empty() {
            encoded.push('0');
        }

        let encoded = encoded.chars().rev().collect();
        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut num = 0u128;

        for c in input.chars() {
            let index = match BASE36_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base36 character").into()),
            };
            num = num * 36 + index as u128;
        }

        let mut decoded = Vec::new();
        while num > 0 {
            decoded.push((num & 0xFF) as u8);
            num >>= 8;
        }

        decoded.reverse();
        Ok(decoded)
    }
}

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub struct Base58 {}

impl Base58 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut num = 0u128;
        for byte in input {
            num = num * 256 + *byte as u128;
        }

        let mut encoded = String::new();
        while num > 0 {
            let remainder = num % 58;
            num /= 58;
            let value = match BASE58_ALPHABET.chars().nth(remainder as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        remainder,
                        BASE58_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
        }

        for byte in input {
            if *byte == 0 {
                let value = BASE58_ALPHABET.chars().nth(0).unwrap();
                encoded.push(value);
            } else {
                break;
            }
        }

        let encoded = encoded.chars().rev().collect();
        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut num = 0u128;

        for c in input.chars() {
            let index = match BASE58_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base58 character").into()),
            };
            num = num * 58 + index as u128;
        }

        let mut decoded = Vec::new();
        while num > 0 {
            decoded.push((num % 256) as u8);
            num /= 256;
        }

        // 添加前导零
        for c in input.chars() {
            if c == BASE58_ALPHABET.chars().nth(0).unwrap() {
                decoded.push(0);
            } else {
                break;
            }
        }

        decoded.reverse();
        Ok(decoded)
    }
}

const BASE62_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct Base62 {}

impl Base62 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut num = 0u128;
        for byte in input {
            num = num * 256 + *byte as u128;
        }

        let mut encoded = String::new();
        while num > 0 {
            let remainder = num % 62;
            num /= 62;
            let value = match BASE62_ALPHABET.chars().nth(remainder as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        remainder,
                        BASE62_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
        }

        for byte in input {
            if *byte == 0 {
                encoded.push(BASE62_ALPHABET.chars().nth(0).unwrap());
            } else {
                break;
            }
        }

        let encoded = encoded.chars().rev().collect();
        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut num = 0u128;

        for c in input.chars() {
            let index = match BASE62_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base62 character").into()),
            };
            num = num * 62 + index as u128;
        }

        let mut decoded = Vec::new();
        while num > 0 {
            decoded.push((num % 256) as u8);
            num /= 256;
        }

        for c in input.chars() {
            if c == BASE62_ALPHABET.chars().nth(0).unwrap() {
                decoded.push(0);
            } else {
                break;
            }
        }

        decoded.reverse();
        Ok(decoded)
    }
}

const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Base64 {}

impl Base64 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut encoded = String::new();
        let mut buffer = 0u32;
        let mut bits_left = 0;

        for byte in input {
            buffer = (buffer << 8) | (*byte as u32);
            bits_left += 8;

            while bits_left >= 6 {
                bits_left -= 6;
                let index = (buffer >> bits_left) & 0b111111;
                let value = match BASE64_ALPHABET.chars().nth(index as usize) {
                    Some(v) => v,
                    None => {
                        return Err(DecodeError::new(&format!(
                            "get value from alphabet failed, index {}, max length {}",
                            index,
                            BASE64_ALPHABET.len(),
                        ))
                        .into())
                    }
                };
                encoded.push(value);
            }
        }

        if bits_left > 0 {
            let index = (buffer << (6 - bits_left)) & 0b111111;
            let value = match BASE64_ALPHABET.chars().nth(index as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        index,
                        BASE64_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
        }

        while encoded.len() % 4 != 0 {
            encoded.push('=');
        }

        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut decoded = Vec::new();
        let mut buffer = 0u32;
        let mut bits_left = 0;

        for c in input.chars() {
            if c == '=' {
                break;
            }

            let index = match BASE64_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base64 character").into()),
            };
            buffer = (buffer << 6) | (index as u32);
            bits_left += 6;

            if bits_left >= 8 {
                bits_left -= 8;
                decoded.push((buffer >> bits_left) as u8);
            }
        }

        Ok(decoded)
    }
}

const BASE85_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

const BASE91_ALPHABET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&()*+,./:;<=>?@[]^_`{|}~\"";

pub struct Base91 {}

impl Base91 {
    pub fn encode(input: &[u8]) -> Result<String> {
        let mut b = 0u32;
        let mut n = 0u32;
        let mut encoded = String::new();

        for byte in input {
            b |= (*byte as u32) << n;
            n += 8;
            if n > 13 {
                let mut v = b & 8191;
                if v > 88 {
                    b >>= 13;
                    n -= 13;
                } else {
                    v = b & 16383;
                    b >>= 14;
                    n -= 14;
                }
                let value = match BASE91_ALPHABET.chars().nth((v % 91) as usize) {
                    Some(v) => v,
                    None => {
                        return Err(DecodeError::new(&format!(
                            "get value from alphabet failed, index {}, max length {}",
                            (v % 91) as usize,
                            BASE91_ALPHABET.len(),
                        ))
                        .into())
                    }
                };
                encoded.push(value);
                let value = match BASE91_ALPHABET.chars().nth((v / 91) as usize) {
                    Some(v) => v,
                    None => {
                        return Err(DecodeError::new(&format!(
                            "get value from alphabet failed, index {}, max length {}",
                            (v / 91) as usize,
                            BASE91_ALPHABET.len(),
                        ))
                        .into())
                    }
                };
                encoded.push(value);
            }
        }

        if n > 0 {
            let value = match BASE91_ALPHABET.chars().nth((b % 91) as usize) {
                Some(v) => v,
                None => {
                    return Err(DecodeError::new(&format!(
                        "get value from alphabet failed, index {}, max length {}",
                        (b % 91) as usize,
                        BASE91_ALPHABET.len(),
                    ))
                    .into())
                }
            };
            encoded.push(value);
            if n > 7 || b > 90 {
                let value = match BASE91_ALPHABET.chars().nth((b / 91) as usize) {
                    Some(v) => v,
                    None => {
                        return Err(DecodeError::new(&format!(
                            "get value from alphabet failed, index {}, max length {}",
                            (b / 91) as usize,
                            BASE91_ALPHABET.len(),
                        ))
                        .into())
                    }
                };
                encoded.push(value);
            }
        }

        Ok(encoded)
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        let mut decoded = Vec::new();
        let mut b = 0u32;
        let mut n = 0u32;
        let mut v = -1;

        for c in input.chars() {
            let d = match BASE91_ALPHABET.find(c) {
                Some(i) => i,
                None => return Err(DecodeError::new("invalid base91 character").into()),
            };
            if v < 0 {
                v = d as i32;
            } else {
                v += (d as i32) * 91;
                b |= (v as u32) << n;
                if (v & 8191) > 88 {
                    n += 13;
                } else {
                    n += 14;
                }
                while n > 7 {
                    decoded.push((b & 255) as u8);
                    b >>= 8;
                    n -= 8;
                }
                v = -1;
            }
        }

        if v + 1 > 0 {
            decoded.push(((b | (v as u32) << n) & 255) as u8);
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base16() -> Result<()> {
        let data = b"Hello";
        let enc = Base16::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base16::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base32() -> Result<()> {
        let data = b"Hello";
        let enc = Base32::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base32::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base36() -> Result<()> {
        let data = b"Hello";
        let enc = Base36::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base36::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base58() -> Result<()> {
        let data = b"Hello";
        let enc = Base58::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base58::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base62() -> Result<()> {
        let data = b"Hello";
        let enc = Base62::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base62::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base64() -> Result<()> {
        let data = b"Hello";
        let enc = Base64::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base64::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base91() -> Result<()> {
        let data = b"Hello";
        let enc = Base91::encode(data)?;
        println!("enc: {}", enc);
        let dec = Base91::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
}
