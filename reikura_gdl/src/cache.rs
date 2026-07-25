use std::num::NonZeroUsize;

use lru::LruCache;

use crate::{AssetName, Audio, Image, Scenario};

pub struct CacheManager {
    pub image: LruCache<AssetName, Image>,
    pub wipe_image: LruCache<AssetName, Image>,
    pub scene: LruCache<AssetName, Scenario>,
    pub voice: LruCache<AssetName, Audio>,
    pub sfx: LruCache<AssetName, Audio>,
    pub bgm: LruCache<AssetName, Audio>,
    pub cdda: Option<LruCache<u8, Audio>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            image: LruCache::new(NonZeroUsize::new(64).unwrap()),
            wipe_image: LruCache::new(NonZeroUsize::new(16).unwrap()),
            scene: LruCache::new(NonZeroUsize::new(32).unwrap()),
            sfx: LruCache::new(NonZeroUsize::new(32).unwrap()),
            bgm: LruCache::new(NonZeroUsize::new(16).unwrap()),
            voice: LruCache::new(NonZeroUsize::new(96).unwrap()),
            cdda: None,
        }
    }
}
