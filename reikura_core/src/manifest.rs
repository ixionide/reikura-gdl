use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};
use reikura_util::encoding::decode_sjis;

const STARTUP_INFO: &str = "[StartUpInfo]";
const COMPANY: &str = "COMPANY";
const KEY: &str = "KEY";
const TITLE: &str = "TITLE";

const GAME_INFO: &str = "[GameInfo]";
const _WND_POS: &str = "WndPos";
const _USER_SETUP: &str = "UserSetup";

const REIKURA_INFO: &str = "[ReikuraInfo]";
const VIEW_SIZE: &str = "ViewSize";
const FONT_PATH: &str = "FontPath";

pub struct Manifest {
    pub suf_path: PathBuf,

    // startupinfo section
    pub company: Option<String>,
    pub key: String,
    pub title: String,

    // gameinfo section. we dont need this
    _wnd_pos: Option<String>,
    _user_setup: Option<String>,

    // reikurainfo section
    pub view_size: (u32, u32),
    pub font_path: Option<PathBuf>,
}

impl Manifest {
    pub fn parse(suf_path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(&suf_path)?;
        let content = decode_sjis(bytes)?;

        let mut root_section = HashMap::new();
        let mut startup_info_section = HashMap::new();
        let mut game_info_section = HashMap::new();
        let mut reikura_info_section = HashMap::new();

        let mut current_section = &mut root_section;

        for line in content.lines().map(str::trim) {
            if line.starts_with('[') && line.ends_with(']') {
                current_section = match line {
                    STARTUP_INFO => &mut startup_info_section,
                    GAME_INFO => &mut game_info_section,
                    REIKURA_INFO => &mut reikura_info_section,
                    _ => &mut root_section,
                };
            } else {
                let Some((key, value)) = line.split_once('=') else {
                    bail!("invalid key value pair: {line}");
                };

                let key = key.trim().to_string();
                let value = value.trim().to_string();

                current_section.insert(key, value);
            }
        }

        let Some(key) = startup_info_section.remove(KEY) else {
            bail!("missing key value in manifest file")
        };

        let title = startup_info_section
            .remove(TITLE)
            .unwrap_or("Reikura".into());
        let view_size = parse_view_size(reikura_info_section.remove(VIEW_SIZE), &key);

        Ok(Self {
            suf_path: suf_path.as_ref().to_owned(),
            company: startup_info_section.remove(COMPANY),
            key,
            title,
            _wnd_pos: None,
            _user_setup: None,
            view_size,
            font_path: reikura_info_section.remove(FONT_PATH).map(Into::into),
        })
    }

    pub fn game_path(&self) -> &Path {
        self.suf_path.parent().unwrap()
    }
}

fn parse_view_size(value: Option<String>, key: &str) -> (u32, u32) {
    let default = match key {
        "KANAOKA" => (800, 600),
        _ => (640, 480),
    };

    let Some((w, h)) = value.as_ref().and_then(|it| it.split_once(',')) else {
        return default;
    };

    let (Ok(w), Ok(h)) = (w.trim().parse(), h.trim().parse()) else {
        return default;
    };

    (w, h)
}
