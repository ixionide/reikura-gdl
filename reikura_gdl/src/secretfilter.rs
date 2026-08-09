pub mod filters;

// shuffled
const BASE36_DIGITS: [u8; 36] = *b"G5FXIL094MPRKWCJ3OEBVA7HQ2SU8Y6TZ1ND";

pub const fn decode_base36_digit(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'A'..=b'Z' => digit - b'A' + 10,
        b'a'..=b'z' => digit - b'a' + 10,
        _ => panic!("invalid digit"),
    }
}

pub fn decode(digits: &mut [u8]) {
    #[rustfmt::skip]
    const MAP_TABLE: [u8; 36] = [
        0x06, 0x21, 0x19, 0x10, 0x08, 0x01, 0x1E, 0x16, 0x1C, 0x07, 0x15, 0x13,
        0x0E, 0x23, 0x12, 0x02, 0x00, 0x17, 0x04, 0x0F, 0x0C, 0x05, 0x09, 0x22,
        0x11, 0x0A, 0x18, 0x0B, 0x1A, 0x1F, 0x1B, 0x14, 0x0D, 0x03, 0x1D, 0x20,
    ];

    pub fn map(digit: u8) -> u8 {
        MAP_TABLE[decode_base36_digit(digit) as usize]
    }

    digits.iter_mut().for_each(|byte| *byte = map(*byte));
}

fn base36_diff(val1: u8, val2: u8) -> u8 {
    let diff = val1.abs_diff(val2) as usize;

    if val1 < val2 {
        BASE36_DIGITS[36 - diff]
    } else {
        BASE36_DIGITS[diff]
    }
}

pub fn update_key(key: &mut [u8], pos: usize, filter: &[u8; 2048]) {
    let filter = &filter[..0x400];
    decode(key);

    for (i, k) in key.iter_mut().enumerate() {
        let index = (*k + filter[(pos + i) % 0x400]) % 36;
        *k = BASE36_DIGITS[index as usize];
    }
}

pub fn get_key(filter: &[u8; 2048]) -> Vec<u8> {
    let hi_digit = base36_diff(filter[0x500], filter[0x100]);
    let lo_digit = base36_diff(filter[0x501], filter[0x101]);
    let hi = decode_base36_digit(hi_digit) << 4;
    let lo = decode_base36_digit(lo_digit);
    let len = (hi | lo) as usize;

    let mut key = vec![0; len];
    for i in 0..len {
        key[i] = base36_diff(filter[0x510 + i], filter[0x110 + i]);
    }

    key
}

#[test]
fn decode_isf() {
    let mut filter = filters::TOSHIAKI;
    decode(&mut filter);
    let mut isf = std::fs::read("src/secretfilter/START.ISF").unwrap();
    let mut key = get_key(&filter);
    let key_len = key.len();

    for (i, b) in &mut isf.iter_mut().enumerate() {
        if i.is_multiple_of(key_len) {
            update_key(&mut key, i, &filter);
        }

        *b ^= key[i % key_len];
    }

    std::fs::write("start.isf", isf).unwrap();
}
