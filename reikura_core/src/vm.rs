use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use reikura_util::{bitset::BitSet, variable::Variables};

use crate::{AssetManager, Config, Manifest, Scenario};

pub struct VmContext {
    pub flags: BitSet,
    // patterns: HashMap<u8, FlagPattern>,
    pub variables: Variables,
    pub characters: HashMap<u8, String>,
    pub wait_time: Option<i32>,
    pub time_counter: Option<(Instant, Duration)>,
}

pub struct Vm {
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub config: Config,
    pub ctx: VmContext,
    pub scene: Scenario,
}
