use crate::bitset::BitSet;

pub struct Flag<B: AsRef<[u8]> + AsMut<[u8]> = Vec<u8>> {
    inner: BitSet<B>,
}

impl Flag {
    pub fn new(count: usize) -> Self {
        Self {
            inner: BitSet::new(count),
        }
    }

    pub fn resize(&mut self, count: usize) {
        self.inner.resize(count);
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> Flag<B> {
    pub fn from_raw(bytes: B, count: usize) -> Self {
        Self {
            inner: BitSet::from_raw(bytes, count),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, index: usize) -> bool {
        self.inner.get(index).unwrap_or(false)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.inner.set(index, value);
    }

    pub fn toggle(&mut self, index: usize) {
        self.inner.toggle(index);
    }
}
