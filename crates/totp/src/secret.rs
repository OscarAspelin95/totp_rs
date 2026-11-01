use base32::{Alphabet, decode, encode};
use errors::AppError;
use rand;
use rand::prelude::*;

const ALPHABET: Alphabet = Alphabet::Crockford;

fn decode_secret(encoded: String) -> Result<Vec<u8>, AppError> {
    let decoded = decode(ALPHABET, &encoded).ok_or_else(|| AppError::Base32Error)?;
    Ok(decoded)
}

fn encode_secret(secret: &[u8]) -> String {
    encode(ALPHABET, &secret)
}

fn random_buf() -> [u8; 20] {
    let mut buf = [0u8; 20];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut buf);

    buf
}

pub fn generate_random_secret() -> String {
    let secret = random_buf();
    encode_secret(&secret)
}

#[cfg(test)]
pub mod test {
    use super::{decode_secret, encode_secret, random_buf};

    #[test]
    fn test_encode_decode() {
        let original_secret = random_buf();

        let encoded_secret = encode_secret(&original_secret);
        let decoded_secret = decode_secret(encoded_secret).unwrap();

        assert_eq!(original_secret, &decoded_secret[..]);
    }
}
