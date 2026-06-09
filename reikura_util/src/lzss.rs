const WINDOW_SIZE: usize = 4096;
const INITIAL_POS: usize = WINDOW_SIZE - 18;
const MIN_MATCH_LENGTH: usize = 3;

pub fn decompress(input: &[u8], output_size: usize) -> Vec<u8> {
    let input_size = input.len();
    let mut window = vec![0; WINDOW_SIZE];
    let mut output = vec![0; output_size];
    let mut window_pos = INITIAL_POS;
    let mut input_pos = 0;
    let mut output_pos = 0;

    while output_pos < output_size && input_pos < input_size {
        let flag = input[input_pos];
        input_pos += 1;

        for shift in 0..8 {
            if ((flag >> shift) & 1) == 1 {
                if output_pos >= output_size || input_pos >= input_size {
                    return output;
                }
                // literal byte
                let byte = input[input_pos];
                output[output_pos] = byte;
                window[window_pos] = byte;

                input_pos += 1;
                output_pos += 1;
                window_pos = (window_pos + 1) % WINDOW_SIZE;
            } else {
                if input_pos + 1 >= input_size {
                    return output;
                }

                let lo = input[input_pos] as usize;
                let hi = input[input_pos + 1] as usize;
                input_pos += 2;

                let mut offset = ((hi & 0xF0) << 4) | lo;
                let length = (hi & 0x0F) + MIN_MATCH_LENGTH;

                for _ in 0..length {
                    if output_pos >= output_size {
                        return output;
                    }

                    let byte = window[offset];
                    output[output_pos] = byte;
                    window[window_pos] = byte;

                    output_pos += 1;
                    window_pos = (window_pos + 1) % WINDOW_SIZE;
                    offset = (offset + 1) % WINDOW_SIZE;
                }
            }
        }
    }

    output
}
