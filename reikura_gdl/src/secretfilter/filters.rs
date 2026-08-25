pub const KANAOKA: [u8; 2048] = *include_bytes!("kanaoka.filter");
pub const KZNR: [u8; 2048] = *include_bytes!("kznr.filter");
pub const PURUC: [u8; 2048] = *include_bytes!("puruc.filter");
pub const PURUCX: [u8; 2048] = *include_bytes!("purucx.filter");
pub const TOSHIAKI: [u8; 2048] = *include_bytes!("toshiaki.filter");
pub const UMINOEX: [u8; 2048] = *include_bytes!("uminoex.filter");
pub const YUKI: [u8; 2048] = *include_bytes!("yuki.filter");

pub fn get_known_filter(title_id: &str) -> Option<[u8; 2048]> {
    let filter = match title_id {
        "KANAOKA" => KANAOKA,
        "KZNR" | "KZNRUS" => KZNR,
        "PURUC" | "PURUCUS" => PURUC,
        "PURUCX" | "PURUCXUS" => PURUC,
        "TOSHIAKI" => TOSHIAKI,
        "UMINOEX" => UMINOEX,
        "YUKI" | "YUKIUS" => YUKI,
        _ => return None,
    };

    Some(filter)
}
