use std::hash::Hasher;
use fxhash::FxHasher;

#[derive(Default)]
pub struct MyHasher {
    hasher: FxHasher,
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}
