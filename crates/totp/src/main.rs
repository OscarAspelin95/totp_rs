use std::time::Duration;

use errors::AppError;
use totp::{generate_random_secret, get_code};

fn main() -> Result<(), AppError> {
    let secret = generate_random_secret();

    loop {
        let (code, seconds_left) = get_code(secret.as_bytes())?;
        println!("{code} - {seconds_left}s left");
        std::thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}
