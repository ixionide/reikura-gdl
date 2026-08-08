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

pub fn decode_filter(filter: &mut [u8; 2048]) {
    #[rustfmt::skip]
    const MAP_TABLE: [u8; 36] = [
        0x06, 0x21, 0x19, 0x10, 0x08, 0x01, 0x1E, 0x16, 0x1C, 0x07, 0x15, 0x13,
        0x0E, 0x23, 0x12, 0x02, 0x00, 0x17, 0x04, 0x0F, 0x0C, 0x05, 0x09, 0x22,
        0x11, 0x0A, 0x18, 0x0B, 0x1A, 0x1F, 0x1B, 0x14, 0x0D, 0x03, 0x1D, 0x20,
    ];

    pub fn map(digit: u8) -> u8 {
        MAP_TABLE[decode_base36_digit(digit) as usize]
    }

    filter.iter_mut().for_each(|byte| *byte = map(*byte));
}

pub fn get_key_len(filter: &[u8; 2048]) -> usize {
    fn base36_diff(val1: u8, val2: u8) -> u8 {
        let diff = val1.abs_diff(val2) as usize;

        let digit = {
            if val1 < val2 {
                BASE36_DIGITS[36 - diff]
            } else {
                BASE36_DIGITS[diff]
            }
        };

        decode_base36_digit(digit)
    }

    let hi = base36_diff(filter[0x500], filter[0x100]) << 4;
    let lo = base36_diff(filter[0x501], filter[0x101]);

    (hi | lo) as usize
}

#[test]
fn decode() {
    let mut filter = filters::YUKI;
    decode_filter(&mut filter);
}
