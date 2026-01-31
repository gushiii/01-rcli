use rand::seq::{IndexedRandom, SliceRandom};

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    digits: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();

    let charsets: [(&bool, &[u8]); 4] = [
        (&upper, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        (&lower, b"abcdefghijklmnopqrstuvwxyz"),
        (&digits, b"0123456789"),
        (&symbol, b"!@#$%^&*_.?"),
    ];

    let active_sets: Vec<&[u8]> = charsets
        .iter()
        .filter(|(enabled, _)| **enabled)
        .map(|(_, set)| *set)
        .collect();

    if active_sets.is_empty() {
        anyhow::bail!("chars won't be empty in this context");
    }

    let mut password: Vec<char> = active_sets
        .iter()
        .map(|set| *set.choose(&mut rng).unwrap() as char)
        .collect();

    let char_pool: Vec<u8> = active_sets
        .iter()
        .flat_map(|set| set.iter())
        .copied()
        .collect();

    password.extend(
        (password.len()..length as usize).map(|_| *char_pool.choose(&mut rng).unwrap() as char),
    );

    password.shuffle(&mut rng);
    println!("{}", password.into_iter().collect::<String>());

    Ok(())
}
