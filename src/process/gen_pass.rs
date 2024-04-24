use rand::seq::SliceRandom;

const UPPER_CASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER_CASE);
        password.push(
            *UPPER_CASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if lower {
        chars.extend_from_slice(LOWER_CASE);
        password.push(
            *LOWER_CASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if number {
        chars.extend_from_slice(NUMBERS);
        password.push(
            *NUMBERS
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if symbol {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    for _ in 0..length - 4 {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c)
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    Ok(password)
}
