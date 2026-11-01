use errors::AppError;

use chrono::prelude::*;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const NUM_DIGITS: u32 = 6;

fn get_counter() -> Result<u64, AppError> {
    let ts = Utc::now().timestamp();

    u64::try_from(ts / 30)
        .map_err(|_| AppError::TimeStampError(format!("Timestamp {ts} cannot be negative")))
}

fn hash_counter(counter: u64, secret: &[u8]) -> Result<u32, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(&counter.to_be_bytes());
    let hash = mac.finalize().into_bytes();

    // RFC 4226
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let bin = u32::from_be_bytes(hash[offset..offset + 4].try_into()?) & 0x7fff_ffff;

    Ok(bin % 10u32.pow(NUM_DIGITS))
}

pub fn get_code(secret: &[u8]) -> Result<u32, AppError> {
    let timestamp = get_counter()?;
    let code = hash_counter(timestamp, secret)?;
    Ok(code)
}
