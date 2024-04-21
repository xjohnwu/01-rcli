use crate::opts::GenPassOpts;
use rand::seq::SliceRandom;

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend(b'A'..=b'Z');
    }

    if opts.lowercase {
        chars.extend(b'a'..=b'z');
    }

    if opts.number {
        chars.extend(b'0'..=b'9');
    }

    if opts.symbol {
        chars.extend(b'!'..=b'/');
        chars.extend(b':'..=b'@');
        chars.extend(b'['..=b'`');
        chars.extend(b'{'..=b'~');
    }

    for _ in 0..opts.length {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c as char)
    }

    println!("{}", password);
    Ok(())
}
