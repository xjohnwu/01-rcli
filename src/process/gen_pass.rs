use crate::opts::GenPassOpts;
use rand::seq::SliceRandom;

const UPPER_CASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER_CASE);
        password.push(
            *UPPER_CASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWER_CASE);
        password.push(
            *LOWER_CASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if opts.number {
        chars.extend_from_slice(NUMBERS);
        password.push(
            *NUMBERS
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    for _ in 0..opts.length - 4 {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c)
    }
    password.shuffle(&mut rng);

    println!("{}", String::from_utf8_lossy(&password));
    Ok(())
}
