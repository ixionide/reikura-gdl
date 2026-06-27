// pub trait Cache {
//     fn mem_size(&self) -> usize;
// }

use std::num::NonZeroUsize;

use lru::LruCache;

use crate::{Audio, Image, ScenarioCache};

pub struct CacheManager {
    pub image: LruCache<String, Image>,
    pub wipe_image: LruCache<String, Image>,
    pub scene: LruCache<String, ScenarioCache>,
    pub voice: LruCache<String, Audio>,
    pub sfx: LruCache<String, Audio>,
    pub bgm: LruCache<String, Audio>,
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
        }
    }
}
