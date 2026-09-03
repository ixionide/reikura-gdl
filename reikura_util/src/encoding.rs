use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct InvalidSJIS(Vec<u8>);

impl Display for InvalidSJIS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to encode/decode sjis string with buffer: {:?}",
            self.0
        )
    }
}

impl std::error::Error for InvalidSJIS {}

#[inline]
pub fn sjis_to_utf8(sjis: &[u8]) -> Result<String, InvalidSJIS> {
    let (cow, _, err) = encoding_rs::SHIFT_JIS.decode(sjis);

    if err {
        return Err(InvalidSJIS(sjis.to_vec()));
    }

    Ok(cow.into_owned())
}

#[inline]
pub fn utf8_to_sjis(utf8: &str) -> Result<Vec<u8>, InvalidSJIS> {
    let (cow, _, err) = encoding_rs::SHIFT_JIS.encode(utf8);

    if err {
        return Err(InvalidSJIS(utf8.as_bytes().to_vec()));
    }

    Ok(cow.into_owned())
}
