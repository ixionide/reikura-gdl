use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct InvalidSJIS(Vec<u8>);

impl Display for InvalidSJIS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "failed to decode sjis string with buffer: {:?}", self.0)
    }
}

impl std::error::Error for InvalidSJIS {}

#[inline]
pub fn decode_sjis(bytes: Vec<u8>) -> Result<String, InvalidSJIS> {
    let (cow, _, err) = encoding_rs::SHIFT_JIS.decode(&bytes);

    if err {
        return Err(InvalidSJIS(bytes));
    }

    Ok(cow.to_string())
}
