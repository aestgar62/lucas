//

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Prompt {
    pub cacheable_prefix: String,
    pub dynamic_suffix: String,
}

impl Prompt {

    pub fn fingerprint(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.cacheable_prefix.hash(&mut hasher);
        self.dynamic_suffix.hash(&mut hasher);
        hasher.finish()
    }
}