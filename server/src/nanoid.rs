use rand::{rngs::StdRng, FromEntropy, Rng};

pub const NO_LOOK_ALIKE: [char; 54] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn default(size: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut result: Vec<u8> = vec![0; size];

    rng.fill(&mut result[..]);

    result
}

pub fn format(random: fn(usize) -> Vec<u8>, alphabet: &[char], size: usize) -> String {
    assert!(
        alphabet.len() <= u8::max_value() as usize,
        "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
    );

    let mask = alphabet.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    debug_assert!(alphabet.len() <= mask + 1);

    let mut id = String::with_capacity(size);

    loop {
        let bytes = random(step);

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

#[macro_export]
macro_rules! nanoid {
    // simple
    () => {
        $crate::nanoid::format($crate::nanoid::default, &$crate::nanoid::NO_LOOK_ALIKE, 6)
    };

    // generate
    ($size:tt) => {
        $crate::nanoid::format(
            $crate::nanoid::default,
            &$crate::nanoid::NO_LOOK_ALIKE,
            $size,
        )
    };

    // custom
    ($size:tt, $alphabet:expr) => {
        $crate::nanoid::format($crate::nanoid::default, $alphabet, $size)
    };

    // complex
    ($size:tt, $alphabet:expr, $random:expr) => {
        $crate::nanoid::format($random, $alphabet, $size)
    };
}
