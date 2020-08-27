use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn nanoid() -> String {
    let alphabet = &[
        '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let size = 6;

    assert!(
        alphabet.len() <= u8::max_value() as usize,
        "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
    );

    let mask = alphabet.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    debug_assert!(alphabet.len() <= mask + 1);

    let mut id = String::with_capacity(size);

    loop {
        let mut rng = StdRng::from_entropy();
        let mut bytes: Vec<u8> = vec![0; step];

        rng.fill(&mut bytes[..]);

        for &byte in &bytes {
            let byte = byte as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    return id;
                }
            }
        }
    }
}
