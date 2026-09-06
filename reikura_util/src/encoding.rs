use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum InvalidSJIS {
    EncodeError(String),
    DecodeError(Vec<u8>),
}

impl Display for InvalidSJIS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidSJIS::EncodeError(str) => {
                write!(f, "failed to endode to sjis with string: {:?}", str)
            }
            InvalidSJIS::DecodeError(buf) => {
                write!(f, "failed to decode to sjis with buffer: {:?}", buf)
            }
        }
    }
}

impl std::error::Error for InvalidSJIS {}

#[inline]
pub fn sjis_to_utf8(sjis: &[u8]) -> Result<String, InvalidSJIS> {
    let (cow, _, err) = encoding_rs::SHIFT_JIS.decode(sjis);

    if err {
        return Err(InvalidSJIS::DecodeError(sjis.to_vec()));
    }

    Ok(cow.into_owned())
}

#[inline]
pub fn utf8_to_sjis(utf8: &str) -> Result<Vec<u8>, InvalidSJIS> {
    let (cow, _, err) = encoding_rs::SHIFT_JIS.encode(utf8);

    if err {
        return Err(InvalidSJIS::EncodeError(utf8.to_owned()));
    }

    Ok(cow.into_owned())
}
