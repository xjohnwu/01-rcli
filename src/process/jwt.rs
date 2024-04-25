use anyhow::Result;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

const SECRET: &str = "secret";

fn parse_duration(input: &str) -> Result<u64> {
    let length = input.len();
    if length < 2 {
        anyhow::bail!("Input too short")
    }

    // Separate the numeric part from the unit part
    let (number, unit) = input.split_at(length - 1);

    // Convert the numeric part to a number
    let number: u64 = number.parse()?;

    // Determine the multiplier based on the unit
    let seconds = match unit {
        "d" => 86400, // 24 * 60 * 60
        "h" => 3600,  // 60 * 60
        "m" => 60,
        "s" => 1,
        _ => anyhow::bail!("Invalid time unit"),
    };

    Ok(number * seconds)
}

pub fn process_jwt_sign(sub: &str, aud: &str, exp: &str) -> Result<String> {
    let claims = Claims {
        sub: sub.to_owned(),
        aud: aud.to_owned(),
        exp: get_current_timestamp() + parse_duration(exp)?,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["device1"]);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    println!("{:?}", token_data);
    Ok(token_data.claims)
}
