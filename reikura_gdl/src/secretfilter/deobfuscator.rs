pub struct Deobfuscator {
    key: Vec<u8>,
    filter: [u8; 2048],
}

impl Deobfuscator {
    pub fn new(mut filter: [u8; 2048]) -> Self {
        decode(&mut filter);
        let key = get_key(&filter);

        Self { key, filter }
    }

    pub fn try_filter_search(executable: &[u8]) -> Option<Self> {
        let mut filter = [0; 2048];
        let start = executable
            .array_windows::<8>()
            .position(|slice| slice == b"UOB0GMVM")?;
        let end = start + 2048;
        filter.copy_from_slice(executable.get(start..end)?);

        Some(Self::new(filter))
    }

    pub fn deobfuscate(&self, data: &mut [u8]) {
        let mut key = self.key.clone();
        let key_len = key.len();

        for (i, b) in data.iter_mut().enumerate() {
            if i.is_multiple_of(key_len) {
                update_key(&mut key, i, &self.filter);
            }

            *b ^= key[i % key_len];
        }
    }
}

// shuffled (0-9)(A-Z)
const BASE36_DIGITS: [u8; 36] = *b"G5FXIL094MPRKWCJ3OEBVA7HQ2SU8Y6TZ1ND";

fn decode_base36_digit(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'A'..=b'Z' => digit - b'A' + 10,
        b'a'..=b'z' => digit - b'a' + 10,
        _ => panic!("invalid digit"),
    }
}

fn decode(digits: &mut [u8]) {
    #[rustfmt::skip]
    const MAP_TABLE: [u8; 36] = [
        0x06, 0x21, 0x19, 0x10, 0x08, 0x01, 0x1E, 0x16, 0x1C, 0x07, 0x15, 0x13,
        0x0E, 0x23, 0x12, 0x02, 0x00, 0x17, 0x04, 0x0F, 0x0C, 0x05, 0x09, 0x22,
        0x11, 0x0A, 0x18, 0x0B, 0x1A, 0x1F, 0x1B, 0x14, 0x0D, 0x03, 0x1D, 0x20,
    ];

    pub fn map(digit: &mut u8) {
        *digit = MAP_TABLE[decode_base36_digit(*digit) as usize];
    }

    digits.iter_mut().for_each(map);
}

fn base36_diff(val1: u8, val2: u8) -> u8 {
    let diff = val1.abs_diff(val2) as usize;

    if val1 < val2 {
        BASE36_DIGITS[36 - diff]
    } else {
        BASE36_DIGITS[diff]
    }
}

fn get_key(filter: &[u8; 2048]) -> Vec<u8> {
    let hi_digit = base36_diff(filter[0x500], filter[0x100]);
    let lo_digit = base36_diff(filter[0x501], filter[0x101]);
    let hi = decode_base36_digit(hi_digit);
    let lo = decode_base36_digit(lo_digit);
    let len = (hi << 4) | lo;

    let mut key = vec![0; len as usize];
    key.iter_mut()
        .enumerate()
        .for_each(|(i, k)| *k = base36_diff(filter[0x510 + i], filter[0x110 + i]));

    key
}

fn update_key(key: &mut [u8], start: usize, filter: &[u8; 2048]) {
    let filter = filter[0..1024].iter().cycle().skip(start);
    decode(key);

    for (k, f) in key.iter_mut().zip(filter) {
        let index = (*k + f) % 36;
        *k = BASE36_DIGITS[index as usize];
    }
}
