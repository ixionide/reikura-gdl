pub const YUKI: [u8; 2048] = *include_bytes!("yuki.filter");
pub const UMINOEX: [u8; 2048] = *include_bytes!("uminoex.filter");
pub const TOSHIAKI: [u8; 2048] = *include_bytes!("toshiaki.filter");

pub fn get_known_filter(title_id: &str) -> Option<[u8; 2048]> {
    match title_id {
        "YUKI" => YUKI,
        "UMINOEX" => UMINOEX,
        "TOSHIAKI" => TOSHIAKI,
        _ => return None,
    }
    .into()
}
