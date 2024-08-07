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
    ///     let enc = Base16::encode(input);
    ///     println!("{}", enc);
    /// }
    /// ```
    pub fn encode(input: &[u8]) -> String {
        let encoded = input
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        encoded
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

pub struct Base32 {}

impl Base32 {
    pub fn encode(input: &[u8]) -> String {
        const BASE32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut encoded = String::new();
        let mut buffer = 0u32;
        let mut bits_left = 0;

        for &byte in input {
            buffer = (buffer << 8) | (byte as u32);
            bits_left += 8;

            while bits_left >= 5 {
                bits_left -= 5;
                let index = (buffer >> bits_left) & 0b11111;
                encoded.push(BASE32_ALPHABET.chars().nth(index as usize).unwrap());
            }
        }

        if bits_left > 0 {
            let index = (buffer << (5 - bits_left)) & 0b11111;
            encoded.push(BASE32_ALPHABET.chars().nth(index as usize).unwrap());
        }

        while encoded.len() % 8 != 0 {
            encoded.push('=');
        }

        encoded
    }
    pub fn decode(input: &str) -> Result<Vec<u8>> {
        const BASE32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base16() -> Result<()> {
        let data = b"Hello";
        let enc = Base16::encode(data);
        println!("enc: {}", enc);
        let dec = Base16::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
    #[test]
    fn base32() -> Result<()> {
        let data = b"Hello";
        let enc = Base32::encode(data);
        println!("enc: {}", enc);
        let dec = Base32::decode(&enc)?;
        assert_eq!(dec, data);
        Ok(())
    }
}
