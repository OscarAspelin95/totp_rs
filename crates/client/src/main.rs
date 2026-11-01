use errors::AppError;
use totp::get_code;

use clap::Parser;

#[derive(Parser)]
pub struct App {
    #[arg(short, long)]
    secret: String,
}

fn main() -> Result<(), AppError> {
    let args = App::parse();
    let code = get_code(args.secret.as_bytes())?;
    println!("{code}");

    Ok(())
}
